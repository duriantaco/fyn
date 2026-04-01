use std::fmt::Write;

use tracing::debug;

use fyn_cache::Cache;
use fyn_client::BaseClientBuilder;
use fyn_configuration::DryRun;
use fyn_distribution_types::{InstalledDist, InstalledDistKind};
use fyn_fs::Simplified;
use fyn_installer::SitePackages;
use fyn_preview::Preview;
use fyn_python::{
    EnvironmentPreference, PythonDownloads, PythonEnvironment, PythonInstallation,
    PythonPreference, PythonRequest,
};
use fyn_requirements::{GroupsSpecification, RequirementsSource};
use fyn_workspace::WorkspaceCache;

use crate::commands::pip::install::pip_install;
use crate::commands::reporters::PythonDownloadReporter;
use crate::commands::{ExitStatus, pip::operations::report_interpreter};
use crate::printer::Printer;
use crate::settings::PipSettings;

/// Upgrade all installed packages in the current environment.
pub(crate) async fn pip_upgrade(
    settings: PipSettings,
    python_downloads: PythonDownloads,
    python_preference: PythonPreference,
    client_builder: &BaseClientBuilder<'_>,
    concurrency: fyn_configuration::Concurrency,
    cache: Cache,
    workspace_cache: WorkspaceCache,
    dry_run: DryRun,
    installer_metadata: bool,
    printer: Printer,
    preview: Preview,
) -> anyhow::Result<ExitStatus> {
    let PipSettings {
        index_locations,
        python,
        install_mirrors,
        system,
        extras,
        groups: _,
        break_system_packages,
        target,
        prefix,
        index_strategy,
        keyring_provider,
        torch_backend,
        build_isolation,
        extra_build_dependencies,
        extra_build_variables,
        build_options,
        allow_empty_requirements: _,
        strict,
        dependency_mode,
        resolution,
        prerelease,
        fork_strategy: _,
        dependency_metadata,
        output_file: _,
        no_strip_extras: _,
        no_strip_markers: _,
        no_annotate: _,
        no_header: _,
        custom_compile_command: _,
        generate_hashes: _,
        config_setting,
        config_settings_package,
        python_version,
        python_platform,
        universal: _,
        exclude_newer,
        no_emit_package: _,
        emit_index_url: _,
        emit_find_links: _,
        emit_build_options: _,
        emit_marker_expression: _,
        emit_index_annotation: _,
        annotation_style: _,
        link_mode,
        compile_bytecode,
        sources,
        hash_checking,
        upgrade,
        reinstall,
    } = settings;

    let environment = if target.is_some() || prefix.is_some() {
        let python_request = python.as_deref().map(PythonRequest::parse);
        let reporter = PythonDownloadReporter::single(printer);

        let installation = PythonInstallation::find_or_download(
            python_request.as_ref(),
            EnvironmentPreference::from_system_flag(system, false),
            python_preference.with_system_flag(system),
            python_downloads,
            client_builder,
            &cache,
            Some(&reporter),
            install_mirrors.python_install_mirror.as_deref(),
            install_mirrors.pypy_install_mirror.as_deref(),
            install_mirrors.python_downloads_json_url.as_deref(),
            preview,
        )
        .await?;
        report_interpreter(&installation, true, printer)?;
        PythonEnvironment::from_installation(installation)
    } else {
        PythonEnvironment::find(
            &python
                .as_deref()
                .map(PythonRequest::parse)
                .unwrap_or_default(),
            EnvironmentPreference::from_system_flag(system, true),
            PythonPreference::default().with_system_flag(system),
            &cache,
            preview,
        )?
    };

    let environment = if let Some(target) = target.clone() {
        debug!(
            "Using `--target` directory at {}",
            target.root().user_display()
        );
        environment.with_target(target)?
    } else if let Some(prefix) = prefix.clone() {
        debug!(
            "Using `--prefix` directory at {}",
            prefix.root().user_display()
        );
        environment.with_prefix(prefix)?
    } else {
        environment
    };

    let site_packages = SitePackages::from_environment(&environment)?;
    let mut requirements = site_packages
        .iter()
        .map(installed_requirement)
        .collect::<anyhow::Result<Vec<_>>>()?;

    requirements.sort_unstable_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
    requirements.dedup_by(|(left_key, _), (right_key, _)| left_key == right_key);

    let requirements = requirements
        .into_iter()
        .map(|(_, requirement)| requirement)
        .collect::<Vec<_>>();

    if requirements.is_empty() {
        writeln!(printer.stderr(), "Nothing to upgrade")?;
        return Ok(ExitStatus::Success);
    }

    Box::pin(pip_install(
        &requirements,
        &[],
        &[],
        &[],
        &[],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        &extras,
        &GroupsSpecification::default(),
        resolution,
        prerelease,
        dependency_mode,
        upgrade,
        index_locations,
        index_strategy,
        torch_backend,
        dependency_metadata,
        keyring_provider,
        client_builder,
        reinstall,
        link_mode,
        compile_bytecode,
        hash_checking,
        installer_metadata,
        &config_setting,
        &config_settings_package,
        build_isolation,
        &extra_build_dependencies,
        &extra_build_variables,
        build_options,
        crate::commands::pip::operations::Modifications::Sufficient,
        python_version,
        python_platform,
        python_downloads,
        install_mirrors,
        strict,
        exclude_newer,
        sources,
        python,
        system,
        break_system_packages,
        target,
        prefix,
        python_preference,
        concurrency,
        cache,
        workspace_cache,
        dry_run,
        printer,
        preview,
    ))
    .await
}

fn installed_requirement(dist: &InstalledDist) -> anyhow::Result<(String, RequirementsSource)> {
    match &dist.kind {
        InstalledDistKind::Registry(registry) => {
            let key = registry.name.to_string();
            let requirement = RequirementsSource::from_package(registry.name.as_ref())?;
            Ok((key, requirement))
        }
        InstalledDistKind::Url(url) => {
            if url.editable {
                let key = format!("-e {}", url.url.as_str());
                let requirement = RequirementsSource::from_editable(url.url.as_str())?;
                Ok((key, requirement))
            } else {
                let key = format!("{} @ {}", url.name, url.url.as_str());
                let requirement = RequirementsSource::from_package(&format!(
                    "{} @ {}",
                    url.name,
                    url.url.as_str()
                ))?;
                Ok((key, requirement))
            }
        }
        InstalledDistKind::EggInfoFile(dist) => {
            let key = dist.name.to_string();
            let requirement = RequirementsSource::from_package(dist.name.as_ref())?;
            Ok((key, requirement))
        }
        InstalledDistKind::EggInfoDirectory(dist) => {
            let key = dist.name.to_string();
            let requirement = RequirementsSource::from_package(dist.name.as_ref())?;
            Ok((key, requirement))
        }
        InstalledDistKind::LegacyEditable(dist) => {
            let key = format!("-e {}", dist.target_url.as_str());
            let requirement = RequirementsSource::from_editable(dist.target_url.as_str())?;
            Ok((key, requirement))
        }
    }
}
