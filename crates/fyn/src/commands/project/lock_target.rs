use itertools::Either;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tracing::info_span;

use fyn_auth::CredentialsCache;
use fyn_configuration::{DependencyGroupsWithDefaults, NoSources};
use fyn_distribution::LoweredRequirement;
use fyn_distribution_types::{Index, IndexLocations, Requirement, RequiresPython};
use fyn_normalize::{GroupName, PackageName};
use fyn_pep508::RequirementOrigin;
use fyn_pypi_types::{Conflicts, SupportedEnvironments, VerbatimParsedUrl};
use fyn_resolver::{Lock, LockVersion, VERSION};
use fyn_scripts::Pep723Script;
use fyn_workspace::dependency_groups::{DependencyGroupError, FlatDependencyGroup};
use fyn_workspace::{Editability, Workspace, WorkspaceMember};

use crate::commands::project::{ProjectError, find_requires_python};

/// A target that can be resolved into a lockfile.
#[derive(Debug, Copy, Clone)]
pub(crate) enum LockTarget<'lock> {
    Workspace(&'lock Workspace),
    Script(&'lock Pep723Script),
}

impl<'lock> From<&'lock Workspace> for LockTarget<'lock> {
    fn from(workspace: &'lock Workspace) -> Self {
        Self::Workspace(workspace)
    }
}

impl<'lock> From<&'lock Pep723Script> for LockTarget<'lock> {
    fn from(script: &'lock Pep723Script) -> Self {
        LockTarget::Script(script)
    }
}

impl<'lock> LockTarget<'lock> {
    /// Return the set of requirements that are attached to the target directly, as opposed to being
    /// attached to any members within the target.
    pub(crate) fn requirements(self) -> Vec<fyn_pep508::Requirement<VerbatimParsedUrl>> {
        match self {
            Self::Workspace(workspace) => workspace.requirements(),
            Self::Script(script) => script.metadata.dependencies.clone().unwrap_or_default(),
        }
    }

    /// Returns the set of overrides for the [`LockTarget`].
    pub(crate) fn overrides(self) -> Vec<fyn_pep508::Requirement<VerbatimParsedUrl>> {
        match self {
            Self::Workspace(workspace) => workspace.overrides(),
            Self::Script(script) => script
                .metadata
                .tool
                .as_ref()
                .and_then(|tool| tool.fyn.as_ref())
                .and_then(|fyn| fyn.override_dependencies.as_ref())
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }

    /// Returns the set of dependency exclusions for the [`LockTarget`].
    pub(crate) fn exclude_dependencies(self) -> Vec<fyn_normalize::PackageName> {
        match self {
            Self::Workspace(workspace) => workspace.exclude_dependencies(),
            Self::Script(script) => script
                .metadata
                .tool
                .as_ref()
                .and_then(|tool| tool.fyn.as_ref())
                .and_then(|fyn| fyn.exclude_dependencies.as_ref())
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }

    /// Returns the set of constraints for the [`LockTarget`].
    pub(crate) fn constraints(self) -> Vec<fyn_pep508::Requirement<VerbatimParsedUrl>> {
        match self {
            Self::Workspace(workspace) => workspace.constraints(),
            Self::Script(script) => script
                .metadata
                .tool
                .as_ref()
                .and_then(|tool| tool.fyn.as_ref())
                .and_then(|fyn| fyn.constraint_dependencies.as_ref())
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }

    /// Returns the set of build constraints for the [`LockTarget`].
    pub(crate) fn build_constraints(self) -> Vec<fyn_pep508::Requirement<VerbatimParsedUrl>> {
        match self {
            Self::Workspace(workspace) => workspace.build_constraints(),
            Self::Script(script) => script
                .metadata
                .tool
                .as_ref()
                .and_then(|tool| tool.fyn.as_ref())
                .and_then(|fyn| fyn.build_constraint_dependencies.as_ref())
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }

    /// Return the dependency groups that are attached to the target directly, as opposed to being
    /// attached to any members within the target.
    pub(crate) fn dependency_groups(
        self,
    ) -> Result<BTreeMap<GroupName, FlatDependencyGroup>, DependencyGroupError> {
        match self {
            Self::Workspace(workspace) => workspace.workspace_dependency_groups(),
            Self::Script(_) => Ok(BTreeMap::new()),
        }
    }

    /// Returns the set of all members within the target.
    pub(crate) fn members_requirements(self) -> impl Iterator<Item = Requirement> + 'lock {
        match self {
            Self::Workspace(workspace) => Either::Left(workspace.members_requirements()),
            Self::Script(_) => Either::Right(std::iter::empty()),
        }
    }

    /// Returns the set of all dependency groups within the target.
    pub(crate) fn group_requirements(self) -> impl Iterator<Item = Requirement> + 'lock {
        match self {
            Self::Workspace(workspace) => Either::Left(workspace.group_requirements()),
            Self::Script(_) => Either::Right(std::iter::empty()),
        }
    }

    /// Return the list of members to include in the [`Lock`].
    pub(crate) fn members(self) -> Vec<PackageName> {
        match self {
            Self::Workspace(workspace) => {
                let mut members = workspace.packages().keys().cloned().collect::<Vec<_>>();
                members.sort();

                // If this is a non-virtual project with a single member, we can omit it from the lockfile.
                // If any members are added or removed, it will inherently mismatch. If the member is
                // renamed, it will also mismatch.
                if members.len() == 1 && !workspace.is_non_project() {
                    members.clear();
                }

                members
            }
            Self::Script(_) => Vec::new(),
        }
    }

    /// Return the list of packages.
    pub(crate) fn packages(self) -> &'lock BTreeMap<PackageName, WorkspaceMember> {
        match self {
            Self::Workspace(workspace) => workspace.packages(),
            Self::Script(_) => {
                static EMPTY: BTreeMap<PackageName, WorkspaceMember> = BTreeMap::new();
                &EMPTY
            }
        }
    }

    /// Return the set of required workspace members, i.e., those that are required by other
    /// members.
    pub(crate) fn required_members(self) -> &'lock BTreeMap<PackageName, Editability> {
        match self {
            Self::Workspace(workspace) => workspace.required_members(),
            Self::Script(_) => {
                static EMPTY: BTreeMap<PackageName, Editability> = BTreeMap::new();
                &EMPTY
            }
        }
    }

    /// Returns the set of supported environments for the [`LockTarget`].
    pub(crate) fn environments(self) -> Option<&'lock SupportedEnvironments> {
        match self {
            Self::Workspace(workspace) => workspace.environments(),
            Self::Script(_) => {
                // TODO(charlie): Add support for environments in scripts.
                None
            }
        }
    }

    /// Returns the set of required platforms for the [`LockTarget`].
    pub(crate) fn required_environments(self) -> Option<&'lock SupportedEnvironments> {
        match self {
            Self::Workspace(workspace) => workspace.required_environments(),
            Self::Script(_) => {
                // TODO(charlie): Add support for environments in scripts.
                None
            }
        }
    }

    /// Returns the set of conflicts for the [`LockTarget`].
    pub(crate) fn conflicts(self) -> Result<Conflicts, ProjectError> {
        match self {
            Self::Workspace(workspace) => Ok(workspace.conflicts()?),
            Self::Script(_) => Ok(Conflicts::empty()),
        }
    }

    /// Return an iterator over the [`Index`] definitions in the [`LockTarget`].
    pub(crate) fn indexes(self) -> impl Iterator<Item = &'lock Index> {
        match self {
            Self::Workspace(workspace) => Either::Left(workspace.indexes().iter().chain(
                workspace.packages().values().flat_map(|member| {
                    member
                        .pyproject_toml()
                        .tool
                        .as_ref()
                        .and_then(|tool| tool.fyn.as_ref())
                        .and_then(|fyn| fyn.index.as_ref())
                        .into_iter()
                        .flatten()
                }),
            )),
            Self::Script(script) => Either::Right(
                script
                    .metadata
                    .tool
                    .as_ref()
                    .and_then(|tool| tool.fyn.as_ref())
                    .and_then(|fyn| fyn.top_level.index.as_deref())
                    .into_iter()
                    .flatten(),
            ),
        }
    }

    /// Return the `Requires-Python` bound for the [`LockTarget`].
    pub(crate) fn requires_python(self) -> Result<Option<RequiresPython>, ProjectError> {
        match self {
            Self::Workspace(workspace) => {
                // When locking, don't try to enforce requires-python bounds that appear on groups
                let groups = DependencyGroupsWithDefaults::none();
                find_requires_python(workspace, &groups)
            }
            Self::Script(script) => Ok(script
                .metadata
                .requires_python
                .as_ref()
                .map(RequiresPython::from_specifiers)),
        }
    }

    /// Return the path to the lock root.
    pub(crate) fn install_path(self) -> &'lock Path {
        match self {
            Self::Workspace(workspace) => workspace.install_path(),
            Self::Script(script) => script.path.parent().unwrap(),
        }
    }

    /// Return the filename of the lockfile, for use in user-facing messages.
    pub(crate) fn lock_filename(self) -> PathBuf {
        PathBuf::from(
            self.lock_path()
                .file_name()
                .expect("lock path should have a file name"),
        )
    }

    /// Return the path to the lockfile.
    ///
    /// Respects the `UV_LOCKFILE` environment variable if set, otherwise
    /// defaults to `fyn.lock` in the workspace root.
    pub(crate) fn lock_path(self) -> PathBuf {
        let lockfile_name = std::env::var(fyn_static::EnvVars::UV_LOCKFILE)
            .ok()
            .filter(|s| !s.is_empty())
            .filter(|s| !s.contains('/') && !s.contains('\\') && !s.contains(".."))
            .unwrap_or_else(|| "fyn.lock".to_string());

        match self {
            Self::Workspace(workspace) => workspace.install_path().join(lockfile_name),
            // `script.py.lock`
            Self::Script(script) => {
                let mut file_name = script
                    .path
                    .file_name()
                    .expect("script path should have a file name")
                    .to_os_string();
                file_name.push(".lock");
                script.path.with_file_name(file_name)
            }
        }
    }

    /// Read the lockfile from the workspace.
    ///
    /// Returns `Ok(None)` if the lockfile does not exist.
    pub(crate) async fn read(self) -> Result<Option<Lock>, ProjectError> {
        let lock_path = self.lock_path();
        match fs_err::tokio::read_to_string(&lock_path).await {
            Ok(encoded) => {
                let result = info_span!("toml::from_str lock", path = %lock_path.display())
                    .in_scope(|| toml::from_str::<Lock>(&encoded));
                match result {
                    Ok(lock) => {
                        // If the lockfile uses an unsupported version, raise an error.
                        if lock.version() != VERSION {
                            return Err(ProjectError::UnsupportedLockVersion(
                                VERSION,
                                lock.version(),
                            ));
                        }
                        Ok(Some(lock))
                    }
                    Err(err) => {
                        // If we failed to parse the lockfile, determine whether it's a supported
                        // version.
                        if let Ok(lock) = toml::from_str::<LockVersion>(&encoded) {
                            if lock.version() != VERSION {
                                return Err(ProjectError::UnparsableLockVersion(
                                    VERSION,
                                    lock.version(),
                                    err,
                                ));
                            }
                        }
                        Err(ProjectError::UvLockParse(err))
                    }
                }
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    /// Read the lockfile from the workspace as bytes.
    pub(crate) async fn read_bytes(self) -> Result<Option<Vec<u8>>, std::io::Error> {
        match fs_err::tokio::read(self.lock_path()).await {
            Ok(encoded) => Ok(Some(encoded)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Write the lockfile to disk.
    pub(crate) async fn commit(self, lock: &Lock) -> Result<(), ProjectError> {
        let encoded = lock.to_toml()?;
        fs_err::tokio::write(self.lock_path(), encoded).await?;
        Ok(())
    }

    /// Lower the requirements for the [`LockTarget`], relative to the target root.
    pub(crate) fn lower(
        self,
        requirements: Vec<fyn_pep508::Requirement<VerbatimParsedUrl>>,
        locations: &IndexLocations,
        sources: &NoSources,
        credentials_cache: &CredentialsCache,
    ) -> Result<Vec<Requirement>, fyn_distribution::MetadataError> {
        match self {
            Self::Workspace(workspace) => {
                let name = workspace
                    .pyproject_toml()
                    .project
                    .as_ref()
                    .map(|project| project.name.clone());

                // We model these as `build-requires`, since, like build requirements, it doesn't define extras
                // or dependency groups.
                let metadata = fyn_distribution::BuildRequires::from_workspace(
                    fyn_pypi_types::BuildRequires {
                        name,
                        requires_dist: requirements,
                    },
                    workspace,
                    locations,
                    sources,
                    credentials_cache,
                )?;

                Ok(metadata
                    .requires_dist
                    .into_iter()
                    .map(|requirement| requirement.with_origin(RequirementOrigin::Workspace))
                    .collect::<Vec<_>>())
            }
            Self::Script(script) => {
                // Collect any `tool.fyn.index` from the script.
                let empty = Vec::default();
                let indexes = script
                    .metadata
                    .tool
                    .as_ref()
                    .and_then(|tool| tool.fyn.as_ref())
                    .and_then(|fyn| fyn.top_level.index.as_deref())
                    .unwrap_or(&empty);

                // Collect any `tool.fyn.sources` from the script.
                let empty = BTreeMap::default();
                let sources_map = script
                    .metadata
                    .tool
                    .as_ref()
                    .and_then(|tool| tool.fyn.as_ref())
                    .and_then(|fyn| fyn.sources.as_ref())
                    .unwrap_or(&empty);

                Ok(requirements
                    .into_iter()
                    .flat_map(|requirement| {
                        // Check if sources should be disabled for this specific package
                        if sources.for_package(&requirement.name) {
                            vec![Ok(Requirement::from(requirement))].into_iter()
                        } else {
                            let requirement_name = requirement.name.clone();
                            LoweredRequirement::from_non_workspace_requirement(
                                requirement,
                                script.path.parent().unwrap(),
                                sources_map,
                                indexes,
                                locations,
                                credentials_cache,
                            )
                            .map(move |requirement| match requirement {
                                Ok(requirement) => Ok(requirement.into_inner()),
                                Err(err) => Err(fyn_distribution::MetadataError::LoweringError(
                                    requirement_name.clone(),
                                    Box::new(err),
                                )),
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                        }
                    })
                    .collect::<Result<_, _>>()?)
            }
        }
    }
}
