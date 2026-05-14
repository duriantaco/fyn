use std::collections::BTreeSet;
use std::path::Path;

use anstream::print;
use anyhow::{Result, bail};
use fyn_cache::Cache;
use fyn_client::BaseClientBuilder;
use fyn_configuration::{
    Concurrency, DependencyGroups, DependencyGroupsWithDefaults, TargetTriple,
};
use fyn_normalize::{DefaultGroups, PackageName};
use fyn_preview::Preview;
use fyn_python::{PythonDownloads, PythonPreference, PythonRequest, PythonVersion};
use fyn_resolver::{Lock, WhyDisplay};
use fyn_scripts::Pep723Script;
use fyn_settings::PythonInstallMirrors;
use fyn_workspace::{DiscoveryOptions, Workspace, WorkspaceCache};

use crate::commands::pip::loggers::DefaultResolveLogger;
use crate::commands::pip::resolution_markers;
use crate::commands::project::lock::{LockMode, LockOperation};
use crate::commands::project::lock_target::LockTarget;
use crate::commands::project::{
    ProjectError, ProjectInterpreter, ScriptInterpreter, UniversalState, default_dependency_groups,
};
use crate::commands::{ExitStatus, diagnostics};
use crate::printer::Printer;
use crate::settings::FrozenSource;
use crate::settings::LockCheck;
use crate::settings::ResolverSettings;

/// Run a command.
pub(crate) async fn why(
    project_dir: &Path,
    package: PackageName,
    groups: DependencyGroups,
    lock_check: LockCheck,
    frozen: Option<FrozenSource>,
    universal: bool,
    depth: u8,
    python_version: Option<PythonVersion>,
    python_platform: Option<TargetTriple>,
    python: Option<String>,
    install_mirrors: PythonInstallMirrors,
    settings: ResolverSettings,
    client_builder: &BaseClientBuilder<'_>,
    script: Option<Pep723Script>,
    python_preference: PythonPreference,
    python_downloads: PythonDownloads,
    concurrency: Concurrency,
    no_config: bool,
    cache: &Cache,
    printer: Printer,
    preview: Preview,
) -> Result<ExitStatus> {
    // Find the project requirements.
    let workspace_cache = WorkspaceCache::default();
    let workspace;
    let target = if let Some(script) = script.as_ref() {
        LockTarget::Script(script)
    } else {
        workspace =
            Workspace::discover(project_dir, &DiscoveryOptions::default(), &workspace_cache)
                .await?;
        LockTarget::Workspace(&workspace)
    };

    // Determine the groups to include.
    let default_groups = match target {
        LockTarget::Workspace(workspace) => default_dependency_groups(workspace.pyproject_toml())?,
        LockTarget::Script(_) => DefaultGroups::default(),
    };
    let groups = groups.with_defaults(default_groups);

    if matches!(target, LockTarget::Script(_)) {
        if let Some(group) = groups.explicit_names().next() {
            return Err(ProjectError::MissingGroupScript(group.clone()).into());
        }
    }

    // Find an interpreter for the project, unless `--frozen` and `--universal` are both set.
    let interpreter = if frozen.is_some() && universal {
        None
    } else {
        Some(match target {
            LockTarget::Script(script) => ScriptInterpreter::discover(
                script.into(),
                python.as_deref().map(PythonRequest::parse),
                client_builder,
                python_preference,
                python_downloads,
                &install_mirrors,
                false,
                no_config,
                Some(false),
                cache,
                printer,
                preview,
            )
            .await?
            .into_interpreter(),
            LockTarget::Workspace(workspace) => ProjectInterpreter::discover(
                workspace,
                project_dir,
                &groups,
                python.as_deref().map(PythonRequest::parse),
                client_builder,
                python_preference,
                python_downloads,
                &install_mirrors,
                false,
                no_config,
                Some(false),
                cache,
                printer,
                preview,
            )
            .await?
            .into_interpreter(),
        })
    };

    // Determine the lock mode.
    let mode = if let Some(frozen_source) = frozen {
        LockMode::Frozen(frozen_source.into())
    } else if let LockCheck::Enabled(lock_check) = lock_check {
        LockMode::Locked(interpreter.as_ref().unwrap(), lock_check)
    } else if matches!(target, LockTarget::Script(_)) && !target.lock_path().is_file() {
        // If we're locking a script, avoid creating a lockfile if it doesn't already exist.
        LockMode::DryRun(interpreter.as_ref().unwrap())
    } else {
        LockMode::Write(interpreter.as_ref().unwrap())
    };

    // Initialize any shared state.
    let state = UniversalState::default();

    // Update the lockfile, if necessary.
    let lock = match Box::pin(
        LockOperation::new(
            mode,
            &settings,
            client_builder,
            &state,
            Box::new(DefaultResolveLogger),
            &concurrency,
            cache,
            &workspace_cache,
            printer,
            preview,
        )
        .execute(target),
    )
    .await
    {
        Ok(result) => result.into_lock(),
        Err(ProjectError::Operation(err)) => {
            return diagnostics::OperationDiagnostic::native_tls(client_builder.is_native_tls())
                .report(err)
                .map_or(Ok(ExitStatus::Failure), |err| Err(err.into()));
        }
        Err(err) => return Err(err.into()),
    };

    // Determine the markers to use for resolution.
    let markers = (!universal).then(|| {
        resolution_markers(
            python_version.as_ref(),
            python_platform.as_ref(),
            interpreter.as_ref().unwrap(),
        )
    });

    validate_groups(&lock, &groups)?;

    let why = WhyDisplay::new(&lock, markers.as_ref(), &package, depth.into(), &groups);
    if why.is_empty() {
        bail!("No dependency path found for `{package}`");
    }

    print!("{why}");

    Ok(ExitStatus::Success)
}

fn validate_groups(lock: &Lock, groups: &DependencyGroupsWithDefaults) -> Result<(), ProjectError> {
    // If no groups were specified, short-circuit.
    if groups.explicit_names().next().is_none() {
        return Ok(());
    }

    let mut known_groups = lock.dependency_groups().keys().collect::<BTreeSet<_>>();
    let members = if lock.members().is_empty() {
        lock.root().into_iter().collect::<Vec<_>>()
    } else {
        lock.packages()
            .iter()
            .filter(|package| lock.members().contains(package.name()))
            .collect::<Vec<_>>()
    };

    for package in members {
        known_groups.extend(package.dependency_groups().keys());
    }

    for group in groups.explicit_names() {
        if !known_groups.contains(group) {
            return Err(ProjectError::MissingGroupProjects(group.clone()));
        }
    }

    Ok(())
}
