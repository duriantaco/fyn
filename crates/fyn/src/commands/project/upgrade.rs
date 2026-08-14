use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use owo_colors::OwoColorize;
use tracing::warn;

use fyn_cache::{Cache, Refresh};
use fyn_client::BaseClientBuilder;
use fyn_configuration::{Concurrency, DependencyGroups, DryRun, ExtrasSpecification, Upgrade};
use fyn_distribution::{ArchiveMetadata, Metadata};
use fyn_distribution_types::Identifier;
use fyn_normalize::{DefaultExtras, PackageName};
use fyn_pep440::{Operator, Version, VersionSpecifier, VersionSpecifiers};
use fyn_pep508::{MarkerTree, Requirement, VerbatimUrl, VersionOrUrl};
use fyn_preview::Preview;
use fyn_pypi_types::{PyProjectToml, ResolutionMetadata, VerbatimParsedUrl};
use fyn_python::{PythonDownloads, PythonPreference, PythonRequest};
use fyn_redacted::DisplaySafeUrl;
use fyn_resolver::MetadataResponse;
use fyn_settings::{MalwareCheckSettings, PythonInstallMirrors};
use fyn_workspace::pyproject::Source;
use fyn_workspace::pyproject_mut::{DependencyTarget, PyProjectTomlMut};
use fyn_workspace::{DiscoveryOptions, VirtualProject, WorkspaceCache, WorkspaceError};

use crate::commands::pip::loggers::DefaultResolveLogger;
use crate::commands::project::add::{AddTarget, PythonTarget, lock_and_sync};
use crate::commands::project::lock::{LockEvent, LockMode, LockOperation, LockResult};
use crate::commands::project::{
    PlatformState, ProjectEnvironment, ProjectError, ProjectInterpreter, UniversalState,
    default_dependency_groups,
};
use crate::commands::{ExitStatus, diagnostics};
use crate::printer::Printer;
use crate::settings::{LockCheck, ResolverInstallerSettings};

/// A production dependency selected for upgrading.
struct SelectedRequirement {
    original_text: String,
    requirement: Requirement<VerbatimParsedUrl>,
    resolved_versions: BTreeSet<Version>,
}

/// A dependency requirement that will be written to `pyproject.toml`.
struct RequirementUpdate {
    original_text: String,
    replacement: Requirement<VerbatimUrl>,
}

/// Upgrade project dependencies, their constraints, the lockfile, and the environment.
#[expect(clippy::too_many_arguments)]
pub(crate) async fn upgrade(
    project_dir: &Path,
    packages: Vec<PackageName>,
    exclude: Vec<PackageName>,
    dry_run: DryRun,
    no_sync: bool,
    python: Option<String>,
    install_mirrors: PythonInstallMirrors,
    refresh: Refresh,
    mut settings: ResolverInstallerSettings,
    client_builder: BaseClientBuilder<'_>,
    python_preference: PythonPreference,
    python_downloads: PythonDownloads,
    installer_metadata: bool,
    concurrency: Concurrency,
    no_config: bool,
    cache: &Cache,
    workspace_cache: &WorkspaceCache,
    printer: Printer,
    preview: Preview,
    malware_settings: MalwareCheckSettings,
) -> Result<ExitStatus> {
    let project =
        match VirtualProject::discover(project_dir, &DiscoveryOptions::default(), workspace_cache)
            .await
        {
            Ok(VirtualProject::Project(project)) => project,
            Ok(VirtualProject::NonProject(_))
            | Err(WorkspaceError::MissingPyprojectToml | WorkspaceError::MissingProject(_)) => {
                bail!("`fyn upgrade` requires a project with a `[project]` table");
            }
            Err(err) => return Err(err.into()),
        };

    let mut requirements = select_requirements(&project, &packages, &exclude)?;
    let selected_packages = requirements
        .iter()
        .map(|selected| selected.requirement.name.clone())
        .collect::<Vec<_>>();
    settings.resolver.upgrade = selected_packages
        .iter()
        .cloned()
        .fold(Upgrade::none(), |upgrade, package| {
            upgrade.combine(Upgrade::package(package))
        });

    let refresh = refresh.combine(Refresh::from(settings.resolver.upgrade.clone()));
    let cache = cache.clone().with_refresh(refresh.clone());
    let groups = DependencyGroups::default().with_defaults(default_dependency_groups(
        project.current_project().pyproject_toml(),
    )?);
    let extras = ExtrasSpecification::default().with_defaults(DefaultExtras::default());
    let project = VirtualProject::Project(project);

    let target = if no_sync || dry_run.enabled() {
        let interpreter = ProjectInterpreter::discover(
            project.workspace(),
            project_dir,
            &groups,
            python.as_deref().map(PythonRequest::parse),
            &client_builder,
            python_preference,
            python_downloads,
            &install_mirrors,
            false,
            no_config,
            None,
            &cache,
            printer,
            preview,
        )
        .await?
        .into_interpreter();
        AddTarget::Project(project, Box::new(PythonTarget::Interpreter(interpreter)))
    } else {
        let environment = ProjectEnvironment::get_or_init(
            project.workspace(),
            &groups,
            python.as_deref().map(PythonRequest::parse),
            &install_mirrors,
            &client_builder,
            python_preference,
            python_downloads,
            false,
            no_config,
            None,
            &cache,
            DryRun::Disabled,
            printer,
            preview,
        )
        .await?
        .into_environment()?;
        AddTarget::Project(project, Box::new(PythonTarget::Environment(environment)))
    };

    let _lock = target
        .acquire_lock()
        .await
        .inspect_err(|err| warn!("Failed to acquire environment lock: {err}"))
        .ok();
    let snapshot = target.snapshot().await?;
    let client_builder = client_builder
        .clone()
        .keyring(settings.resolver.keyring_provider);

    // Resolve against an in-memory manifest with blocking constraints relaxed. The user's
    // manifest and lockfile remain untouched until all proposed constraints have been validated.
    let mut relaxed_toml = PyProjectTomlMut::from_toml(
        &target_project(&target).pyproject_toml().raw,
        DependencyTarget::PyProjectToml,
    )?;
    for selected in &requirements {
        let relaxed = into_verbatim_requirement(
            relax_requirement(&selected.requirement),
            &selected.requirement.name,
        )?;
        if relaxed_toml.replace_dependency(&relaxed, false)?.is_none() {
            bail!(
                "Dependency `{}` was not found in `project.dependencies`",
                selected.requirement.name
            );
        }
    }
    let relaxed_content = relaxed_toml.to_string();
    let resolution_workspace_cache = WorkspaceCache::default();
    let pyproject_path = target_project(&target).root().join("pyproject.toml");
    let pyproject = PyProjectToml::from_toml(&relaxed_content, pyproject_path.display())?;
    if pyproject
        .project
        .as_ref()
        .is_some_and(|project| project.version.is_none())
    {
        bail!("`fyn upgrade` does not support projects with dynamic versions yet");
    }
    let metadata = ResolutionMetadata::parse_pyproject_toml(pyproject, None)?;
    let metadata = Metadata::from_workspace(
        metadata,
        target_project(&target).root(),
        None,
        &settings.resolver.index_locations,
        settings.resolver.sources.clone(),
        &resolution_workspace_cache,
        client_builder.credentials_cache(),
    )
    .await?;

    let state = UniversalState::default();
    let distribution_id = DisplaySafeUrl::from_file_path(target_project(&target).root())
        .map_err(|()| anyhow::anyhow!("Project root is not a valid file URL"))?
        .distribution_id();
    state.index().distributions().done(
        distribution_id,
        Arc::new(MetadataResponse::Found(ArchiveMetadata::from(metadata))),
    );
    let result = match Box::pin(
        LockOperation::new(
            LockMode::DryRun(target.interpreter()),
            &settings.resolver,
            &client_builder,
            &state,
            Box::new(DefaultResolveLogger),
            &concurrency,
            &cache,
            &resolution_workspace_cache,
            printer,
            preview,
        )
        .with_refresh(&refresh)
        .execute((&target).into()),
    )
    .await
    {
        Ok(result) => result,
        Err(ProjectError::Operation(err)) => {
            return diagnostics::OperationDiagnostic::native_tls(client_builder.is_native_tls())
                .report(err)
                .map_or(Ok(ExitStatus::Failure), |err| Err(err.into()));
        }
        Err(err) => return Err(err.into()),
    };

    let install_path = target_project(&target).workspace().install_path();
    for resolved_package in result.lock().packages() {
        let Some(selected) = requirements
            .iter_mut()
            .find(|selected| resolved_package.name() == &selected.requirement.name)
        else {
            continue;
        };
        if resolved_package.index(install_path)?.is_some()
            && package_applies_to_marker(resolved_package, selected.requirement.marker)
            && let Some(version) = resolved_package.version()
        {
            selected.resolved_versions.insert(version.clone());
        }
    }

    let mut final_toml = PyProjectTomlMut::from_toml(
        &target_project(&target).pyproject_toml().raw,
        DependencyTarget::PyProjectToml,
    )?;
    let mut updates = Vec::new();
    for selected in &requirements {
        let proposed = propose_requirement(&selected.requirement, &selected.resolved_versions)?;
        if proposed == selected.requirement {
            continue;
        }
        let replacement = into_verbatim_requirement(proposed, &selected.requirement.name)?;
        if final_toml
            .replace_dependency(&replacement, false)?
            .is_none()
        {
            bail!(
                "Dependency `{}` was not found in `project.dependencies`",
                selected.requirement.name
            );
        }
        updates.push(RequirementUpdate {
            original_text: selected.original_text.clone(),
            replacement,
        });
    }

    if dry_run.enabled() {
        render_changes(&result, &selected_packages, dry_run, printer)?;
        for update in updates {
            writeln!(
                printer.stderr(),
                "Would update requirement: `{}` -> `{}`",
                update.original_text,
                update.replacement
            )?;
        }
        writeln!(printer.stderr(), "{}", "No changes were made".bold())?;
        return Ok(ExitStatus::Success);
    }

    let content = final_toml.to_string();
    if let Err(err) = target.write(&content) {
        let _ = snapshot.revert();
        return Err(err.into());
    }
    let target = match target.update(&content) {
        Ok(target) => target,
        Err(err) => {
            let _ = snapshot.revert();
            return Err(err.into());
        }
    };

    // Restore both the manifest and lockfile if the operation is interrupted.
    let _ = ctrlc::set_handler({
        let snapshot = snapshot.clone();
        move || {
            let _ = snapshot.revert();
            #[expect(clippy::exit, clippy::cast_possible_wrap)]
            std::process::exit(if cfg!(windows) {
                0xC000_013A_u32 as i32
            } else {
                130
            });
        }
    });

    let sync_state = PlatformState::default();
    let lock_state = sync_state.fork();
    let operation = Box::pin(lock_and_sync(
        target,
        &mut final_toml,
        &[],
        lock_state,
        sync_state,
        LockCheck::Disabled,
        false,
        false,
        false,
        false,
        false,
        false,
        Vec::new(),
        Vec::new(),
        &extras,
        &groups,
        true,
        None,
        false,
        Vec::new(),
        &settings,
        &client_builder,
        installer_metadata,
        &concurrency,
        &cache,
        printer,
        preview,
        &malware_settings,
    ))
    .await;

    if let Err(err) = operation {
        let _ = snapshot.revert();
        return match err {
            ProjectError::Operation(err) => {
                diagnostics::OperationDiagnostic::native_tls(client_builder.is_native_tls())
                    .report(err)
                    .map_or(Ok(ExitStatus::Failure), |err| Err(err.into()))
            }
            err => Err(err.into()),
        };
    }

    render_changes(&result, &selected_packages, DryRun::Disabled, printer)?;
    for update in updates {
        writeln!(
            printer.stderr(),
            "Updated requirement: `{}` -> `{}`",
            update.original_text,
            update.replacement
        )?;
    }
    if no_sync {
        writeln!(
            printer.stderr(),
            "{}{} Lockfile updated (sync skipped).",
            "success".green().bold(),
            ":".bold()
        )?;
    } else {
        writeln!(
            printer.stderr(),
            "{}{} Dependencies upgraded successfully.",
            "success".green().bold(),
            ":".bold()
        )?;
    }

    Ok(ExitStatus::Success)
}

fn target_project(target: &AddTarget) -> &VirtualProject {
    let AddTarget::Project(project, _) = target else {
        unreachable!("`fyn upgrade` only supports projects");
    };
    project
}

/// Select unique production dependency declarations targeted by `fyn upgrade`.
fn select_requirements(
    project: &fyn_workspace::ProjectWorkspace,
    packages: &[PackageName],
    exclude: &[PackageName],
) -> Result<Vec<SelectedRequirement>> {
    if project.workspace().packages().len() != 1 {
        bail!("`fyn upgrade` does not support workspaces with multiple members yet");
    }

    let explicit = !packages.is_empty();
    let dependencies = project
        .current_project()
        .project()
        .dependencies
        .as_deref()
        .unwrap_or_default();
    let pyproject_path = project.project_root().join("pyproject.toml");
    let mut selected = Vec::new();
    let mut found = BTreeSet::new();
    for dependency in dependencies {
        let requirement =
            Requirement::<VerbatimParsedUrl>::from_str(dependency).with_context(|| {
                format!(
                    "Failed to parse dependency `{dependency}` from `project.dependencies` in `{}`",
                    pyproject_path.display()
                )
            })?;
        if exclude.contains(&requirement.name)
            || (explicit && !packages.contains(&requirement.name))
        {
            continue;
        }
        found.insert(requirement.name.clone());
        selected.push(SelectedRequirement {
            original_text: dependency.clone(),
            requirement,
            resolved_versions: BTreeSet::new(),
        });
    }

    for package in packages
        .iter()
        .filter(|package| !exclude.contains(*package))
    {
        if !found.contains(package) {
            bail!("Dependency `{package}` was not found in `project.dependencies`");
        }
    }
    if selected.is_empty() {
        bail!("No dependencies selected for upgrade");
    }

    let mut seen = BTreeSet::new();
    for requirement in &selected {
        let package = &requirement.requirement.name;
        if !seen.insert(package.clone()) {
            bail!("Dependency `{package}` is declared multiple times in `project.dependencies`");
        }
        validate_requirement(project, &requirement.requirement)?;
    }

    // Preserve explicit CLI order while keeping manifest order for an all-package upgrade.
    if explicit {
        selected.sort_by_key(|selected| {
            packages
                .iter()
                .position(|package| package == &selected.requirement.name)
                .unwrap_or(usize::MAX)
        });
    }
    Ok(selected)
}

fn validate_requirement(
    project: &fyn_workspace::ProjectWorkspace,
    requirement: &Requirement<VerbatimParsedUrl>,
) -> Result<()> {
    let package = &requirement.name;
    if matches!(requirement.version_or_url, Some(VersionOrUrl::Url(_))) {
        bail!("Dependency `{package}` is a direct URL requirement and cannot be upgraded");
    }
    if package == project.project_name() {
        bail!("Dependency `{package}` refers to the current project and cannot be upgraded");
    }

    let sources = project
        .current_project()
        .pyproject_toml()
        .tool_fyn()
        .and_then(|fyn| fyn.sources.as_ref())
        .and_then(|sources| sources.inner().get(package))
        .or_else(|| project.workspace().sources().get(package));
    if sources.is_some_and(|sources| {
        sources.iter().any(|source| {
            source_is_applicable(source, requirement.marker)
                && matches!(source, Source::Git { rev: Some(_), .. })
        })
    }) {
        bail!(
            "Dependency `{package}` is pinned to a Git revision and cannot be upgraded commit-to-commit"
        );
    }
    if sources.is_some_and(|sources| {
        sources.iter().any(|source| {
            source_is_applicable(source, requirement.marker)
                && !matches!(source, Source::Registry { .. })
        })
    }) {
        bail!(
            "Dependency `{package}` uses a non-registry source in `tool.fyn.sources` and cannot be upgraded"
        );
    }
    Ok(())
}

fn source_is_applicable(source: &Source, requirement_marker: MarkerTree) -> bool {
    let extra = requirement_marker.top_level_extra_name();
    source
        .extra()
        .is_none_or(|target| extra.as_deref() == Some(target))
        && source.group().is_none()
        && !source.marker().is_disjoint(requirement_marker)
}

fn package_applies_to_marker(
    package: &fyn_resolver::Package,
    requirement_marker: MarkerTree,
) -> bool {
    package.fork_markers().is_empty()
        || package
            .fork_markers()
            .iter()
            .any(|fork_marker| !fork_marker.pep508().is_disjoint(requirement_marker))
}

fn into_verbatim_requirement(
    requirement: Requirement<VerbatimParsedUrl>,
    package: &PackageName,
) -> Result<Requirement<VerbatimUrl>> {
    let Requirement {
        name,
        extras,
        version_or_url,
        marker,
        origin,
    } = requirement;
    let version_or_url = match version_or_url {
        Some(VersionOrUrl::VersionSpecifier(specifiers)) => {
            Some(VersionOrUrl::VersionSpecifier(specifiers))
        }
        Some(VersionOrUrl::Url(_)) => {
            bail!("Dependency `{package}` is a direct URL requirement and cannot be upgraded");
        }
        None => None,
    };
    Ok(Requirement::<VerbatimUrl> {
        name,
        extras,
        version_or_url,
        marker,
        origin,
    })
}

fn render_changes(
    result: &LockResult,
    packages: &[PackageName],
    dry_run: DryRun,
    printer: Printer,
) -> Result<()> {
    let events = match result {
        LockResult::Changed(previous, lock) => {
            LockEvent::detect_changes(previous.as_ref(), lock, dry_run)
                .filter(|event| packages.contains(event.package()))
                .collect::<Vec<_>>()
        }
        LockResult::Unchanged(_) => Vec::new(),
    };
    for package in packages {
        if let Some(event) = events.iter().find(|event| event.package() == package) {
            writeln!(printer.stderr(), "{event}")?;
        } else {
            writeln!(printer.stderr(), "No version change for {package}")?;
        }
    }
    Ok(())
}

/// Return a requirement that admits every applicable resolved version.
fn propose_requirement(
    requirement: &Requirement<VerbatimParsedUrl>,
    resolved_versions: &BTreeSet<Version>,
) -> Result<Requirement<VerbatimParsedUrl>> {
    if resolved_versions.is_empty() {
        return Ok(requirement.clone());
    }
    let Some(VersionOrUrl::VersionSpecifier(specifiers)) = &requirement.version_or_url else {
        return Ok(requirement.clone());
    };
    if resolved_versions
        .iter()
        .all(|version| specifiers.contains(version))
    {
        return Ok(requirement.clone());
    }

    let specifiers = specifiers
        .iter()
        .cloned()
        .map(|specifier| rewrite_specifier(specifier, resolved_versions))
        .collect::<Result<VersionSpecifiers>>()?;
    if !resolved_versions
        .iter()
        .all(|version| specifiers.contains(version))
    {
        let versions = resolved_versions
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("`, `");
        bail!(
            "Dependency `{}` resolved to version(s) `{versions}` which cannot be represented by the upgraded requirement",
            requirement.name
        );
    }
    let mut proposed = requirement.clone();
    proposed.version_or_url = Some(VersionOrUrl::VersionSpecifier(specifiers));
    Ok(proposed)
}

fn rewrite_specifier(
    specifier: VersionSpecifier,
    resolved_versions: &BTreeSet<Version>,
) -> Result<VersionSpecifier> {
    if resolved_versions
        .iter()
        .all(|version| specifier.contains(version))
    {
        return Ok(specifier);
    }
    let (Some(lowest), Some(highest)) = (resolved_versions.first(), resolved_versions.last())
    else {
        return Ok(specifier);
    };

    Ok(match specifier.operator() {
        Operator::GreaterThan
        | Operator::GreaterThanEqual
        | Operator::NotEqual
        | Operator::NotEqualStar => specifier,
        Operator::TildeEqual => VersionSpecifier::from_version(
            Operator::TildeEqual,
            compatible_version_at_precision(lowest, specifier.version().release().len())?,
        )?,
        Operator::Equal => VersionSpecifier::equals_version(lowest.clone()),
        Operator::EqualStar => VersionSpecifier::equals_star_version(
            only_release_at_precision(lowest, specifier.version().release().len())
                .context("Cannot rewrite a version constraint without a release segment")?,
        ),
        Operator::ExactEqual => {
            VersionSpecifier::from_version(Operator::ExactEqual, lowest.clone())?
        }
        Operator::LessThan => VersionSpecifier::less_than_version(increment_version_at_precision(
            highest,
            specifier.version().release().len(),
        )?),
        Operator::LessThanEqual => VersionSpecifier::from_version(
            Operator::LessThanEqual,
            highest.clone().without_local(),
        )?,
    })
}

fn compatible_version_at_precision(version: &Version, precision: usize) -> Result<Version> {
    let release = version
        .release()
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(precision)
        .collect::<Vec<_>>();
    if release.is_empty() {
        bail!("Cannot rewrite a version constraint without a release segment");
    }
    Ok(version.clone().with_release(release).without_local())
}

fn only_release_at_precision(version: &Version, precision: usize) -> Option<Version> {
    let release = version
        .release()
        .iter()
        .copied()
        .chain(std::iter::repeat(0))
        .take(precision)
        .collect::<Vec<_>>();
    (!release.is_empty()).then(|| Version::new(release).with_epoch(version.epoch()))
}

fn increment_version_at_precision(version: &Version, precision: usize) -> Result<Version> {
    let projected = only_release_at_precision(version, precision)
        .context("Cannot rewrite a version constraint without a release segment")?;
    let mut release = projected.release().to_vec();
    let segment_index = release.len();
    let Some(last) = release.last_mut() else {
        bail!("Cannot rewrite a version constraint without a release segment");
    };
    let segment = *last;
    *last = segment.checked_add(1).with_context(|| {
        format!(
            "Cannot expand version `{version}` at release segment {segment_index} (`{segment}`) beyond its maximum value"
        )
    })?;
    Ok(projected.with_release(release))
}

/// Remove upper and exact constraints while retaining lower bounds and exclusions.
fn relax_requirement(
    requirement: &Requirement<VerbatimParsedUrl>,
) -> Requirement<VerbatimParsedUrl> {
    let mut relaxed = requirement.clone();
    let Some(VersionOrUrl::VersionSpecifier(specifiers)) = &requirement.version_or_url else {
        return relaxed;
    };
    let specifiers = specifiers
        .iter()
        .filter_map(|specifier| match specifier.operator() {
            Operator::GreaterThan
            | Operator::GreaterThanEqual
            | Operator::NotEqual
            | Operator::NotEqualStar => Some(specifier.clone()),
            Operator::TildeEqual => Some(VersionSpecifier::greater_than_equal_version(
                specifier.version().clone(),
            )),
            Operator::Equal
            | Operator::EqualStar
            | Operator::ExactEqual
            | Operator::LessThan
            | Operator::LessThanEqual => None,
        })
        .collect::<VersionSpecifiers>();
    relaxed.version_or_url = if specifiers.is_empty() {
        None
    } else {
        Some(VersionOrUrl::VersionSpecifier(specifiers))
    };
    relaxed
}

#[cfg(test)]
mod tests {
    use super::{increment_version_at_precision, propose_requirement, relax_requirement};
    use std::collections::BTreeSet;
    use std::str::FromStr;

    use fyn_pep440::Version;
    use fyn_pep508::Requirement;
    use fyn_pypi_types::VerbatimParsedUrl;

    fn resolved_versions(versions: &[&str]) -> BTreeSet<Version> {
        versions
            .iter()
            .map(|version| Version::from_str(version).expect("valid version"))
            .collect()
    }

    #[test]
    fn relax_requirement_preserves_lower_bounds_and_metadata() {
        let requirement = Requirement::<VerbatimParsedUrl>::from_str(
            "Requests[security]>=1,>1.5,!=2,!=2.1.*,==2.5,<=3,<4 ; python_version >= '3.12'",
        )
        .expect("valid requirement");

        let relaxed = relax_requirement(&requirement);

        assert_eq!(
            relaxed.to_string(),
            "requests[security]>=1,>1.5,!=2,!=2.1.* ; python_full_version >= '3.12'"
        );
    }

    #[test]
    fn propose_requirement_preserves_operator_style() {
        for (requirement, version, expected) in [
            ("requests==1.2.3", "2.4.5", "requests==2.4.5"),
            ("requests===1.2.3", "2.4.5", "requests===2.4.5"),
            ("requests~=1.2", "2.4.5", "requests~=2.4"),
            ("requests==1.2.*", "2.4.5", "requests==2.4.*"),
            ("requests<2", "2.4.5", "requests<3"),
            ("requests<=2", "2.4.5", "requests<=2.4.5"),
        ] {
            let requirement =
                Requirement::<VerbatimParsedUrl>::from_str(requirement).expect("valid requirement");
            let versions = resolved_versions(&[version]);

            let proposed =
                propose_requirement(&requirement, &versions).expect("valid requirement update");

            assert_eq!(proposed.to_string(), expected);
        }
    }

    #[test]
    fn propose_requirement_only_rewrites_blocking_specifiers() {
        let requirement = Requirement::<VerbatimParsedUrl>::from_str("requests>=1,<2,<4")
            .expect("valid requirement");

        let proposed = propose_requirement(&requirement, &resolved_versions(&["2.4.0"]))
            .expect("valid requirement update");

        assert_eq!(proposed.to_string(), "requests>=1,<3,<4");
    }

    #[test]
    fn propose_requirement_admits_multiple_resolved_versions() {
        for (requirement, versions, expected) in [
            ("requests<2", &["1.5.0", "2.4.0"][..], "requests<3"),
            ("requests~=1.2", &["2.4", "2.5"][..], "requests~=2.4"),
        ] {
            let requirement =
                Requirement::<VerbatimParsedUrl>::from_str(requirement).expect("valid requirement");

            let proposed = propose_requirement(&requirement, &resolved_versions(versions))
                .expect("resolved versions can be represented");

            assert_eq!(proposed.to_string(), expected);
        }
    }

    #[test]
    fn propose_requirement_rejects_unrepresentable_constraint() {
        let requirement = Requirement::<VerbatimParsedUrl>::from_str("requests!=2.4,<2")
            .expect("valid requirement");

        let error = propose_requirement(&requirement, &resolved_versions(&["2.4"]))
            .expect_err("the exclusion must remain in place");

        assert!(error.to_string().contains("cannot be represented"));
    }

    #[test]
    fn propose_requirement_preserves_suffixes_and_strips_local_upper_bound() {
        for (requirement, version, expected) in [
            (
                "requests~=1.2",
                "1!2.4rc1.post2.dev3+local",
                "requests~=1!2.4rc1.post2.dev3",
            ),
            ("requests<=1.2.3", "2.4.5+local", "requests<=2.4.5"),
        ] {
            let requirement =
                Requirement::<VerbatimParsedUrl>::from_str(requirement).expect("valid requirement");

            let proposed = propose_requirement(&requirement, &resolved_versions(&[version]))
                .expect("valid requirement update");

            assert_eq!(proposed.to_string(), expected);
        }
    }

    #[test]
    fn increment_version_reports_upper_bound_overflow() {
        let version = Version::new([1, 2, u64::MAX]);

        let error = increment_version_at_precision(&version, 3)
            .expect_err("maximum release segment cannot be incremented");

        assert!(error.to_string().contains("beyond its maximum value"));
    }
}
