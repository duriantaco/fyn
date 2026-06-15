use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use itertools::Itertools as _;
use owo_colors::OwoColorize;

use crate::commands::ExitStatus;
use crate::commands::diagnostics;
use crate::commands::pip::loggers::DefaultResolveLogger;
use crate::commands::pip::resolution_markers;
use crate::commands::project::default_dependency_groups;
use crate::commands::project::install_target::InstallTarget;
use crate::commands::project::lock::{LockMode, LockOperation};
use crate::commands::project::lock_target::LockTarget;
use crate::commands::project::{
    ProjectError, ProjectInterpreter, ScriptInterpreter, UniversalState,
};
use crate::commands::reporters::AuditReporter;
use crate::printer::Printer;
use crate::settings::{FrozenSource, LockCheck, ResolverSettings};

use anyhow::{Result, anyhow};
use fyn_audit::service::{VulnerabilityServiceFormat, osv};
use fyn_audit::types::{Dependency, Finding, VulnerabilityID};
use fyn_cache::Cache;
use fyn_client::{BaseClientBuilder, CachedClient};
use fyn_configuration::{
    Concurrency, DependencyGroups, DependencyGroupsWithDefaults, ExtrasSpecification, TargetTriple,
};
use fyn_normalize::{DefaultExtras, DefaultGroups, PackageName};
use fyn_pep440::Version;
use fyn_preview::{Preview, PreviewFeature};
use fyn_python::{PythonDownloads, PythonPreference, PythonVersion};
use fyn_resolver::{Lock, WhyDisplay};
use fyn_scripts::Pep723Script;
use fyn_settings::PythonInstallMirrors;
use fyn_warnings::warn_user;
use fyn_workspace::{DiscoveryOptions, Workspace, WorkspaceCache};
use rustc_hash::FxHashSet;
use tracing::trace;

pub(crate) async fn audit(
    project_dir: &Path,
    extras: ExtrasSpecification,
    groups: DependencyGroups,
    lock_check: LockCheck,
    frozen: Option<FrozenSource>,
    script: Option<Pep723Script>,
    python_version: Option<PythonVersion>,
    python_platform: Option<TargetTriple>,
    install_mirrors: PythonInstallMirrors,
    settings: ResolverSettings,
    client_builder: BaseClientBuilder<'_>,
    python_preference: PythonPreference,
    python_downloads: PythonDownloads,
    concurrency: Concurrency,
    no_config: bool,
    cache: Cache,
    printer: Printer,
    preview: Preview,
    service: VulnerabilityServiceFormat,
    service_url: Option<String>,
    explain: bool,
    direct_only: bool,
    ignore: Vec<VulnerabilityID>,
    ignore_until_fixed: Vec<VulnerabilityID>,
) -> Result<ExitStatus> {
    // Check if the audit feature is in preview
    if !preview.is_enabled(PreviewFeature::Audit) {
        warn_user!(
            "`fyn audit` is experimental and may change without warning. Pass `--preview-features {}` to disable this warning.",
            PreviewFeature::Audit
        );
    }

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

    // Determine the extras to include.
    let default_extras = match &target {
        LockTarget::Workspace(_) => DefaultExtras::default(),
        LockTarget::Script(_) => DefaultExtras::default(),
    };
    let extras = extras.with_defaults(default_extras);

    // Determine whether we're performing a universal audit.
    let universal = python_version.is_none() && python_platform.is_none();

    // Find an interpreter for the project, unless we're performing a frozen audit with a universal target.
    let interpreter = if frozen.is_some() && universal {
        None
    } else {
        Some(match target {
            LockTarget::Script(script) => ScriptInterpreter::discover(
                script.into(),
                None,
                &client_builder,
                python_preference,
                python_downloads,
                &install_mirrors,
                false,
                no_config,
                Some(false),
                &cache,
                printer,
                preview,
            )
            .await?
            .into_interpreter(),
            LockTarget::Workspace(workspace) => ProjectInterpreter::discover(
                workspace,
                project_dir,
                &groups,
                None,
                &client_builder,
                python_preference,
                python_downloads,
                &install_mirrors,
                false,
                no_config,
                Some(false),
                &cache,
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
            &client_builder,
            &state,
            Box::new(DefaultResolveLogger),
            &concurrency,
            &cache,
            &WorkspaceCache::default(),
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

    let target = match &target {
        LockTarget::Workspace(workspace) => {
            if workspace.is_non_project() {
                InstallTarget::NonProjectWorkspace {
                    workspace,
                    lock: &lock,
                }
            } else {
                InstallTarget::Workspace {
                    workspace,
                    lock: &lock,
                }
            }
        }
        LockTarget::Script(script) => InstallTarget::Script {
            script,
            lock: &lock,
        },
    };

    // Validate that the set of requested extras and development groups are defined in the lockfile.
    target.validate_extras(&extras)?;
    target.validate_groups(&groups)?;

    // Determine the markers to use for resolution.
    let markers = (!universal).then(|| {
        resolution_markers(
            python_version.as_ref(),
            python_platform.as_ref(),
            interpreter.as_ref().unwrap(),
        )
    });

    let auditable = lock.packages_for_audit(&extras, &groups, direct_only);

    // Perform the audit.
    let reporter = AuditReporter::from(printer);

    let dependencies: Vec<Dependency> = auditable
        .iter()
        .map(|(name, version)| Dependency::new((*name).clone(), (*version).clone()))
        .collect();
    let base_client = client_builder.build()?;
    let all_findings = match service {
        VulnerabilityServiceFormat::Osv => {
            let osv_url = if let Some(service_url) = service_url.as_deref() {
                service_url
                    .parse()
                    .map_err(|err| anyhow!("Invalid OSV service base URL `{service_url}`: {err}"))?
            } else {
                osv::API_BASE.clone()
            };
            let client = CachedClient::new(base_client);
            let service = osv::Osv::new(client, Some(osv_url), concurrency, cache);
            trace!("Auditing {n} dependencies against OSV", n = auditable.len());
            service.query_batch(&dependencies, osv::Filter::All).await?
        }
    };

    reporter.on_audit_complete();

    let mut matched_ignores: FxHashSet<&VulnerabilityID> = FxHashSet::default();
    let all_findings: Vec<_> = all_findings
        .into_iter()
        .filter(|finding| match finding {
            Finding::Vulnerability(vulnerability) => {
                if let Some(id) = ignore.iter().find(|id| vulnerability.matches(id)) {
                    matched_ignores.insert(id);
                    return false;
                }
                if let Some(id) = ignore_until_fixed
                    .iter()
                    .find(|id| vulnerability.matches(id))
                {
                    matched_ignores.insert(id);
                    if vulnerability.fix_versions.is_empty() {
                        return false;
                    }
                }
                true
            }
            Finding::ProjectStatus(_) => true,
        })
        .collect();

    for id in ignore.iter().chain(ignore_until_fixed.iter()) {
        if !matched_ignores.contains(id) {
            warn_user!(
                "Ignored vulnerability `{}` does not match any vulnerability in the project",
                id.as_str()
            );
        }
    }

    let explanations = if explain {
        audit_explanations(&lock, markers.as_ref(), &groups, &all_findings)
    } else {
        BTreeMap::new()
    };

    let display = AuditResults {
        printer,
        n_packages: auditable.len(),
        findings: all_findings,
        explanations,
    };
    display.render()
}

fn audit_explanations(
    lock: &Lock,
    markers: Option<&fyn_pypi_types::ResolverMarkerEnvironment>,
    groups: &DependencyGroupsWithDefaults,
    findings: &[Finding],
) -> BTreeMap<(PackageName, Version), Vec<String>> {
    let mut explanations = BTreeMap::new();

    for finding in findings {
        let Finding::Vulnerability(vulnerability) = finding else {
            continue;
        };

        let key = (
            vulnerability.dependency.name().clone(),
            vulnerability.dependency.version().clone(),
        );
        explanations.entry(key).or_insert_with(|| {
            WhyDisplay::for_package(
                lock,
                markers,
                vulnerability.dependency.name(),
                vulnerability.dependency.version(),
                usize::MAX,
                groups,
            )
            .paths()
            .to_vec()
        });
    }

    explanations
}

struct AuditResults {
    printer: Printer,
    n_packages: usize,
    findings: Vec<Finding>,
    explanations: BTreeMap<(PackageName, Version), Vec<String>>,
}

impl AuditResults {
    fn render(&self) -> Result<ExitStatus> {
        let (vulns, statuses): (Vec<_>, Vec<_>) =
            self.findings.iter().partition_map(|finding| match finding {
                Finding::Vulnerability(vuln) => itertools::Either::Left(vuln),
                Finding::ProjectStatus(status) => itertools::Either::Right(status),
            });

        let vuln_banner = if !vulns.is_empty() {
            let s = if vulns.len() == 1 { "y" } else { "ies" };
            format!("{} known vulnerabilit{s}", vulns.len())
                .yellow()
                .to_string()
        } else {
            "no known vulnerabilities".bold().to_string()
        };

        let status_banner = if !statuses.is_empty() {
            let s = if statuses.len() == 1 { "" } else { "es" };
            format!(
                "{} adverse project status{}",
                statuses.len().to_string().yellow(),
                s
            )
        } else {
            "no adverse project statuses".bold().to_string()
        };

        writeln!(
            self.printer.stderr(),
            "Found {vuln_banner} and {status_banner} in {packages}",
            packages = format!(
                "{npackages} {label}",
                npackages = self.n_packages,
                label = if self.n_packages == 1 {
                    "package"
                } else {
                    "packages"
                }
            )
            .bold()
        )?;

        let has_findings = !vulns.is_empty() || !statuses.is_empty();

        if !vulns.is_empty() {
            writeln!(self.printer.stdout_important(), "\nVulnerabilities:\n")?;

            // Group vulnerabilities by (dependency name, version).
            let groups = vulns
                .into_iter()
                .chunk_by(|vuln| (vuln.dependency.name(), vuln.dependency.version()));

            for (dependency, vulns) in &groups {
                let vulns: Vec<_> = vulns.collect();
                let (name, version) = dependency;

                writeln!(
                    self.printer.stdout_important(),
                    "{name_version} has {n} known vulnerabilit{ies}:\n",
                    name_version = format!("{name} {version}").bold(),
                    n = vulns.len(),
                    ies = if vulns.len() == 1 { "y" } else { "ies" },
                )?;

                for vuln in vulns {
                    writeln!(
                        self.printer.stdout_important(),
                        "- {id}: {description}",
                        id = vuln.best_id().as_str().bold(),
                        description = vuln.summary.as_deref().unwrap_or("No summary provided"),
                    )?;

                    if let Some(paths) = self.explanations.get(&(
                        vuln.dependency.name().clone(),
                        vuln.dependency.version().clone(),
                    )) {
                        if paths.is_empty() {
                            writeln!(
                                self.printer.stdout_important(),
                                "\n  Dependency path unavailable for the selected audit view"
                            )?;
                        } else {
                            writeln!(self.printer.stdout_important(), "\n  Included because:")?;
                            for path in paths {
                                writeln!(self.printer.stdout_important(), "  {path}")?;
                            }
                        }
                    }

                    if vuln.fix_versions.is_empty() {
                        writeln!(
                            self.printer.stdout_important(),
                            "\n  No fix versions available\n"
                        )?;
                    } else {
                        writeln!(
                            self.printer.stdout_important(),
                            "\n  Fixed in: {}\n",
                            vuln.fix_versions
                                .iter()
                                .map(std::string::ToString::to_string)
                                .join(", ")
                                .blue()
                        )?;
                    }

                    if let Some(link) = &vuln.link {
                        writeln!(
                            self.printer.stdout_important(),
                            "  Advisory information: {link}\n",
                            link = link.as_str().blue()
                        )?;
                    }
                }
            }
        }

        if !statuses.is_empty() {
            writeln!(self.printer.stdout_important(), "\nAdverse statuses:\n")?;

            // NOTE: Nothing here yet, since we don't actually produce
            // any adverse project statuses at the moment.
        }

        if has_findings {
            Ok(ExitStatus::Failure)
        } else {
            Ok(ExitStatus::Success)
        }
    }
}
