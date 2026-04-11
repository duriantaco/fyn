//! Avoid cyclic crate dependencies between [resolver][`fyn_resolver`],
//! [installer][`fyn_installer`] and [build][`fyn_build`] through [`BuildDispatch`]
//! implementing [`BuildContext`].

use std::ffi::{OsStr, OsString};
use std::path::Path;

use anyhow::{Context, Result};
use futures::FutureExt;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use thiserror::Error;
use tracing::{debug, instrument, trace};

use fyn_build_backend::check_direct_build;
use fyn_build_frontend::{SourceBuild, SourceBuildContext};
use fyn_cache::Cache;
use fyn_client::RegistryClient;
use fyn_configuration::{
    BuildKind, BuildOptions, Constraints, IndexStrategy, NoSources, Reinstall,
};
use fyn_configuration::{BuildOutput, Concurrency};
use fyn_distribution::DistributionDatabase;
use fyn_distribution_filename::DistFilename;
use fyn_distribution_types::{
    CachedDist, ConfigSettings, DependencyMetadata, ExtraBuildRequires, ExtraBuildVariables,
    Identifier, IndexCapabilities, IndexLocations, IsBuildBackendError, Name,
    PackageConfigSettings, Requirement, Resolution, SourceDist, VersionOrUrlRef,
};
use fyn_git::GitResolver;
use fyn_installer::{InstallationStrategy, Installer, Plan, Planner, Preparer, SitePackages};
use fyn_preview::Preview;
use fyn_pypi_types::Conflicts;
use fyn_python::{Interpreter, PythonEnvironment};
use fyn_resolver::{
    ExcludeNewer, FlatIndex, Flexibility, InMemoryIndex, Manifest, OptionsBuilder,
    PythonRequirement, Resolver, ResolverEnvironment,
};
use fyn_types::{
    AnyErrorBuild, BuildArena, BuildContext, BuildIsolation, BuildStack, EmptyInstalledPackages,
    HashStrategy, InFlight,
};
use fyn_workspace::WorkspaceCache;

#[derive(Debug, Error)]
pub enum BuildDispatchError {
    #[error(transparent)]
    BuildFrontend(#[from] AnyErrorBuild),

    #[error(transparent)]
    Tags(#[from] fyn_platform_tags::TagsError),

    #[error(transparent)]
    Resolve(#[from] fyn_resolver::ResolveError),

    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Prepare(#[from] fyn_installer::PrepareError),
}

impl IsBuildBackendError for BuildDispatchError {
    fn is_build_backend_error(&self) -> bool {
        match self {
            Self::Tags(_)
            | Self::Resolve(_)
            | Self::Join(_)
            | Self::Anyhow(_)
            | Self::Prepare(_) => false,
            Self::BuildFrontend(err) => err.is_build_backend_error(),
        }
    }
}

/// The main implementation of [`BuildContext`], used by the CLI, see [`BuildContext`]
/// documentation.
pub struct BuildDispatch<'a> {
    client: &'a RegistryClient,
    cache: &'a Cache,
    constraints: &'a Constraints,
    interpreter: &'a Interpreter,
    index_locations: &'a IndexLocations,
    index_strategy: IndexStrategy,
    flat_index: &'a FlatIndex,
    shared_state: SharedState,
    dependency_metadata: &'a DependencyMetadata,
    build_isolation: BuildIsolation<'a>,
    extra_build_requires: &'a ExtraBuildRequires,
    extra_build_variables: &'a ExtraBuildVariables,
    link_mode: fyn_install_wheel::LinkMode,
    build_options: &'a BuildOptions,
    config_settings: &'a ConfigSettings,
    config_settings_package: &'a PackageConfigSettings,
    hasher: &'a HashStrategy,
    exclude_newer: ExcludeNewer,
    source_build_context: SourceBuildContext,
    build_extra_env_vars: FxHashMap<OsString, OsString>,
    sources: NoSources,
    workspace_cache: WorkspaceCache,
    concurrency: Concurrency,
    preview: Preview,
}

impl<'a> BuildDispatch<'a> {
    pub fn new(
        client: &'a RegistryClient,
        cache: &'a Cache,
        constraints: &'a Constraints,
        interpreter: &'a Interpreter,
        index_locations: &'a IndexLocations,
        flat_index: &'a FlatIndex,
        dependency_metadata: &'a DependencyMetadata,
        shared_state: SharedState,
        index_strategy: IndexStrategy,
        config_settings: &'a ConfigSettings,
        config_settings_package: &'a PackageConfigSettings,
        build_isolation: BuildIsolation<'a>,
        extra_build_requires: &'a ExtraBuildRequires,
        extra_build_variables: &'a ExtraBuildVariables,
        link_mode: fyn_install_wheel::LinkMode,
        build_options: &'a BuildOptions,
        hasher: &'a HashStrategy,
        exclude_newer: ExcludeNewer,
        sources: NoSources,
        workspace_cache: WorkspaceCache,
        concurrency: Concurrency,
        preview: Preview,
    ) -> Self {
        Self {
            client,
            cache,
            constraints,
            interpreter,
            index_locations,
            flat_index,
            shared_state,
            dependency_metadata,
            index_strategy,
            config_settings,
            config_settings_package,
            build_isolation,
            extra_build_requires,
            extra_build_variables,
            link_mode,
            build_options,
            hasher,
            exclude_newer,
            source_build_context: SourceBuildContext::new(concurrency.builds_semaphore.clone()),
            build_extra_env_vars: FxHashMap::default(),
            sources,
            workspace_cache,
            concurrency,
            preview,
        }
    }

    /// Set the environment variables to be used when building a source distribution.
    #[must_use]
    pub fn with_build_extra_env_vars<I, K, V>(mut self, sdist_build_env_variables: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.build_extra_env_vars = sdist_build_env_variables
            .into_iter()
            .map(|(key, value)| (key.as_ref().to_owned(), value.as_ref().to_owned()))
            .collect();
        self
    }
}

#[allow(refining_impl_trait)]
impl BuildContext for BuildDispatch<'_> {
    type SourceDistBuilder = SourceBuild;

    async fn interpreter(&self) -> &Interpreter {
        self.interpreter
    }

    fn cache(&self) -> &Cache {
        self.cache
    }

    fn git(&self) -> &GitResolver {
        &self.shared_state.git
    }

    fn build_arena(&self) -> &BuildArena<SourceBuild> {
        &self.shared_state.build_arena
    }

    fn capabilities(&self) -> &IndexCapabilities {
        &self.shared_state.capabilities
    }

    fn dependency_metadata(&self) -> &DependencyMetadata {
        self.dependency_metadata
    }

    fn build_options(&self) -> &BuildOptions {
        self.build_options
    }

    fn build_isolation(&self) -> BuildIsolation<'_> {
        self.build_isolation
    }

    fn config_settings(&self) -> &ConfigSettings {
        self.config_settings
    }

    fn config_settings_package(&self) -> &PackageConfigSettings {
        self.config_settings_package
    }

    fn sources(&self) -> &NoSources {
        &self.sources
    }

    fn locations(&self) -> &IndexLocations {
        self.index_locations
    }

    fn workspace_cache(&self) -> &WorkspaceCache {
        &self.workspace_cache
    }

    fn extra_build_requires(&self) -> &ExtraBuildRequires {
        self.extra_build_requires
    }

    fn extra_build_variables(&self) -> &ExtraBuildVariables {
        self.extra_build_variables
    }

    async fn resolve<'data>(
        &'data self,
        requirements: &'data [Requirement],
        build_stack: &'data BuildStack,
    ) -> Result<Resolution, BuildDispatchError> {
        let python_requirement = PythonRequirement::from_interpreter(self.interpreter);
        let marker_env = self.interpreter.resolver_marker_environment();
        let tags = self.interpreter.tags()?;

        let resolver = Resolver::new(
            Manifest::simple(requirements.to_vec()).with_constraints(self.constraints.clone()),
            OptionsBuilder::new()
                .exclude_newer(self.exclude_newer.clone())
                .index_strategy(self.index_strategy)
                .build_options(self.build_options.clone())
                .flexibility(Flexibility::Fixed)
                .build(),
            &python_requirement,
            ResolverEnvironment::specific(marker_env),
            self.interpreter.markers(),
            // Conflicting groups only make sense when doing universal resolution.
            Conflicts::empty(),
            Some(tags),
            self.flat_index,
            &self.shared_state.index,
            self.hasher,
            self,
            EmptyInstalledPackages,
            DistributionDatabase::new(
                self.client,
                self,
                self.concurrency.downloads_semaphore.clone(),
            )
            .with_build_stack(build_stack),
        )?;
        let resolution = Resolution::from(resolver.resolve().await.with_context(|| {
            format!(
                "No solution found when resolving: {}",
                requirements
                    .iter()
                    .map(|requirement| format!("`{requirement}`"))
                    .join(", ")
            )
        })?);
        Ok(resolution)
    }

    #[instrument(
        skip(self, resolution, venv),
        fields(
            resolution = resolution.distributions().map(ToString::to_string).join(", "),
            venv = ?venv.root()
        )
    )]
    async fn install<'data>(
        &'data self,
        resolution: &'data Resolution,
        venv: &'data PythonEnvironment,
        build_stack: &'data BuildStack,
    ) -> Result<Vec<CachedDist>, BuildDispatchError> {
        debug!(
            "Installing in {} in {}",
            resolution
                .distributions()
                .map(ToString::to_string)
                .join(", "),
            venv.root().display(),
        );

        // Determine the current environment markers.
        let tags = self.interpreter.tags()?;

        // Determine the set of installed packages.
        let site_packages = SitePackages::from_environment(venv)?;

        let Plan {
            cached,
            remote,
            reinstalls,
            extraneous: _,
        } = Planner::new(resolution).build(
            site_packages,
            InstallationStrategy::Permissive,
            &Reinstall::default(),
            self.build_options,
            self.hasher,
            self.index_locations,
            self.config_settings,
            self.config_settings_package,
            self.extra_build_requires(),
            self.extra_build_variables,
            self.cache(),
            venv,
            tags,
        )?;

        // Nothing to do.
        if remote.is_empty() && cached.is_empty() && reinstalls.is_empty() {
            debug!("No build requirements to install for build");
            return Ok(vec![]);
        }

        // Verify that none of the missing distributions are already in the build stack.
        for dist in &remote {
            let id = dist.distribution_id();
            if build_stack.contains(&id) {
                return Err(BuildDispatchError::BuildFrontend(
                    fyn_build_frontend::Error::CyclicBuildDependency(dist.name().clone()).into(),
                ));
            }
        }

        // Download any missing distributions.
        let wheels = if remote.is_empty() {
            vec![]
        } else {
            let preparer = Preparer::new(
                self.cache,
                tags,
                self.hasher,
                self.build_options,
                DistributionDatabase::new(
                    self.client,
                    self,
                    self.concurrency.downloads_semaphore.clone(),
                )
                .with_build_stack(build_stack),
            );

            debug!(
                "Downloading and building requirement{} for build: {}",
                if remote.len() == 1 { "" } else { "s" },
                remote.iter().map(ToString::to_string).join(", ")
            );

            preparer
                .prepare(remote, &self.shared_state.in_flight, resolution)
                .await?
        };

        // Remove any unnecessary packages.
        if !reinstalls.is_empty() {
            let layout = venv.interpreter().layout();
            for dist_info in &reinstalls {
                let summary = fyn_installer::uninstall(dist_info, &layout)
                    .await
                    .context("Failed to uninstall build dependencies")?;
                debug!(
                    "Uninstalled {} ({} file{}, {} director{})",
                    dist_info.name(),
                    summary.file_count,
                    if summary.file_count == 1 { "" } else { "s" },
                    summary.dir_count,
                    if summary.dir_count == 1 { "y" } else { "ies" },
                );
            }
        }

        // Install the resolved distributions.
        let mut wheels = wheels.into_iter().chain(cached).collect::<Vec<_>>();
        if !wheels.is_empty() {
            debug!(
                "Installing build requirement{}: {}",
                if wheels.len() == 1 { "" } else { "s" },
                wheels.iter().map(ToString::to_string).join(", ")
            );
            wheels = Installer::new(venv, self.preview)
                .with_link_mode(self.link_mode)
                .with_cache(self.cache)
                .install(wheels)
                .await
                .context("Failed to install build dependencies")?;
        }

        Ok(wheels)
    }

    #[instrument(skip_all, fields(version_id = version_id, subdirectory = ?subdirectory))]
    async fn setup_build<'data>(
        &'data self,
        source: &'data Path,
        subdirectory: Option<&'data Path>,
        install_path: &'data Path,
        version_id: Option<&'data str>,
        dist: Option<&'data SourceDist>,
        sources: &'data NoSources,
        build_kind: BuildKind,
        build_output: BuildOutput,
        mut build_stack: BuildStack,
    ) -> Result<SourceBuild, fyn_build_frontend::Error> {
        let dist_name = dist.map(fyn_distribution_types::Name::name);
        let dist_version = dist
            .map(fyn_distribution_types::DistributionMetadata::version_or_url)
            .and_then(|version| match version {
                VersionOrUrlRef::Version(version) => Some(version),
                VersionOrUrlRef::Url(_) => None,
            });

        // Note we can only prevent builds by name for packages with names
        // unless all builds are disabled.
        if self
            .build_options
            .no_build_requirement(dist_name)
            // We always allow editable builds
            && !matches!(build_kind, BuildKind::Editable)
        {
            let err = if let Some(dist) = dist {
                fyn_build_frontend::Error::NoSourceDistBuild(dist.name().clone())
            } else {
                fyn_build_frontend::Error::NoSourceDistBuilds
            };
            return Err(err);
        }

        // Push the current distribution onto the build stack, to prevent cyclic dependencies.
        if let Some(dist) = dist {
            build_stack.insert(dist.distribution_id());
        }

        // Get package-specific config settings if available; otherwise, use global settings.
        let config_settings = if let Some(name) = dist_name {
            if let Some(package_settings) = self.config_settings_package.get(name) {
                package_settings.clone().merge(self.config_settings.clone())
            } else {
                self.config_settings.clone()
            }
        } else {
            self.config_settings.clone()
        };

        // Get package-specific environment variables if available.
        let mut environment_variables = self.build_extra_env_vars.clone();
        if let Some(name) = dist_name {
            if let Some(package_vars) = self.extra_build_variables.get(name) {
                environment_variables.extend(
                    package_vars
                        .iter()
                        .map(|(key, value)| (OsString::from(key), OsString::from(value))),
                );
            }
        }

        let builder = SourceBuild::setup(
            source,
            subdirectory,
            install_path,
            dist_name,
            dist_version,
            self.interpreter,
            self,
            self.source_build_context.clone(),
            version_id,
            self.index_locations,
            sources.clone(),
            self.workspace_cache(),
            config_settings,
            self.build_isolation,
            self.extra_build_requires,
            &build_stack,
            build_kind,
            environment_variables,
            build_output,
            self.client.credentials_cache(),
        )
        .boxed_local()
        .await?;
        Ok(builder)
    }

    async fn direct_build<'data>(
        &'data self,
        source: &'data Path,
        subdirectory: Option<&'data Path>,
        output_dir: &'data Path,
        sources: NoSources,
        build_kind: BuildKind,
        version_id: Option<&'data str>,
    ) -> Result<Option<DistFilename>, BuildDispatchError> {
        let source_tree = if let Some(subdir) = subdirectory {
            source.join(subdir)
        } else {
            source.to_path_buf()
        };

        // Only perform the direct build if the backend is fyn in a compatible version.
        let source_tree_str = source_tree.display().to_string();
        let identifier = version_id.unwrap_or_else(|| &source_tree_str);
        if let Err(reason) = check_direct_build(&source_tree, fyn_version::version()) {
            trace!("Requirements for direct build not matched because {reason}");
            return Ok(None);
        }

        debug!("Performing direct build for {identifier}");

        let output_dir = output_dir.to_path_buf();
        let filename = tokio::task::spawn_blocking(move || -> Result<_> {
            let filename = match build_kind {
                BuildKind::Wheel => {
                    let wheel = fyn_build_backend::build_wheel(
                        &source_tree,
                        &output_dir,
                        None,
                        fyn_version::version(),
                        sources.is_none(),
                    )?;
                    DistFilename::WheelFilename(wheel)
                }
                BuildKind::Sdist => {
                    let source_dist = fyn_build_backend::build_source_dist(
                        &source_tree,
                        &output_dir,
                        fyn_version::version(),
                        sources.is_none(),
                    )?;
                    DistFilename::SourceDistFilename(source_dist)
                }
                BuildKind::Editable => {
                    let wheel = fyn_build_backend::build_editable(
                        &source_tree,
                        &output_dir,
                        None,
                        fyn_version::version(),
                        sources.is_none(),
                    )?;
                    DistFilename::WheelFilename(wheel)
                }
            };
            Ok(filename)
        })
        .await??;

        Ok(Some(filename))
    }
}

/// Shared state used during resolution and installation.
///
/// All elements are `Arc`s, so we can clone freely.
#[derive(Default, Clone)]
pub struct SharedState {
    /// The resolved Git references.
    git: GitResolver,
    /// The discovered capabilities for each registry index.
    capabilities: IndexCapabilities,
    /// The fetched package versions and metadata.
    index: InMemoryIndex,
    /// The downloaded distributions.
    in_flight: InFlight,
    /// Build directories for any PEP 517 builds executed during resolution or installation.
    build_arena: BuildArena<SourceBuild>,
}

impl SharedState {
    /// Fork the [`SharedState`], creating a new in-memory index and in-flight cache.
    ///
    /// State that is universally applicable (like the Git resolver and index capabilities)
    /// are retained.
    #[must_use]
    pub fn fork(&self) -> Self {
        Self {
            git: self.git.clone(),
            capabilities: self.capabilities.clone(),
            build_arena: self.build_arena.clone(),
            ..Default::default()
        }
    }

    /// Return the [`GitResolver`] used by the [`SharedState`].
    pub fn git(&self) -> &GitResolver {
        &self.git
    }

    /// Return the [`InMemoryIndex`] used by the [`SharedState`].
    pub fn index(&self) -> &InMemoryIndex {
        &self.index
    }

    /// Return the [`InFlight`] used by the [`SharedState`].
    pub fn in_flight(&self) -> &InFlight {
        &self.in_flight
    }

    /// Return the [`IndexCapabilities`] used by the [`SharedState`].
    pub fn capabilities(&self) -> &IndexCapabilities {
        &self.capabilities
    }

    /// Return the [`BuildArena`] used by the [`SharedState`].
    pub fn build_arena(&self) -> &BuildArena<SourceBuild> {
        &self.build_arena
    }
}
