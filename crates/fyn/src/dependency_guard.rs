use std::collections::BTreeSet;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::{Context, bail};
use serde::Serialize;
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tracing::debug;

use fyn_distribution_types::{BuiltDist, CachedDist, Dist, InstalledDist, Name, SourceDist};
use fyn_pypi_types::ParsedUrl;
use fyn_settings::DependencyGuardProvider;

use crate::settings::DependencyGuardSettings;

const DEPENDENCY_GUARD_SCHEMA_VERSION: u8 = 1;

#[derive(Debug, Serialize)]
pub(crate) struct DependencyGuardPayload {
    schema_version: u8,
    socket_min_score: u8,
    remote: Vec<DependencyGuardPackage>,
    cached: Vec<DependencyGuardPackage>,
    reinstalls: Vec<DependencyGuardInstalled>,
    extraneous: Vec<DependencyGuardInstalled>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DependencyGuardPackage {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    distribution_kind: &'static str,
    source_kind: &'static str,
    requires_prepare: bool,
    editable: bool,
    virtual_project: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purl: Option<String>,
}

#[derive(Debug, Serialize)]
struct DependencyGuardInstalled {
    name: String,
    version: String,
    path: String,
}

impl DependencyGuardPayload {
    pub(crate) fn from_plan(
        socket_min_score: u8,
        cached: &[CachedDist],
        remote: &[Arc<Dist>],
        reinstalls: &[InstalledDist],
        extraneous: &[InstalledDist],
    ) -> Self {
        Self {
            schema_version: DEPENDENCY_GUARD_SCHEMA_VERSION,
            socket_min_score,
            remote: remote
                .iter()
                .map(|dist| DependencyGuardPackage::from_remote(dist.as_ref()))
                .collect(),
            cached: cached
                .iter()
                .map(DependencyGuardPackage::from_cached)
                .collect(),
            reinstalls: reinstalls
                .iter()
                .map(DependencyGuardInstalled::from_installed)
                .collect(),
            extraneous: extraneous
                .iter()
                .map(DependencyGuardInstalled::from_installed)
                .collect(),
        }
    }

    fn socket_targets(&self) -> BTreeSet<&str> {
        self.remote
            .iter()
            .chain(self.cached.iter())
            .filter_map(|package| package.purl.as_deref())
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.remote.is_empty()
            && self.cached.is_empty()
            && self.reinstalls.is_empty()
            && self.extraneous.is_empty()
    }
}

impl DependencyGuardPackage {
    fn from_remote(dist: &Dist) -> Self {
        match dist {
            Dist::Built(BuiltDist::Registry(registry)) => Self {
                name: registry.name().to_string(),
                version: Some(registry.best_wheel().filename.version.to_string()),
                distribution_kind: "built",
                source_kind: "registry",
                requires_prepare: true,
                editable: false,
                virtual_project: false,
                index: Some(registry.best_wheel().index.to_string()),
                url: None,
                path: None,
                purl: Some(socket_purl(
                    registry.name(),
                    &registry.best_wheel().filename.version,
                )),
            },
            Dist::Built(BuiltDist::DirectUrl(dist)) => Self {
                name: dist.name().to_string(),
                version: Some(dist.filename.version.to_string()),
                distribution_kind: "built",
                source_kind: "direct-url",
                requires_prepare: true,
                editable: false,
                virtual_project: false,
                index: None,
                url: Some(dist.url.to_string()),
                path: None,
                purl: None,
            },
            Dist::Built(BuiltDist::Path(dist)) => Self {
                name: dist.name().to_string(),
                version: Some(dist.filename.version.to_string()),
                distribution_kind: "built",
                source_kind: "path",
                requires_prepare: true,
                editable: false,
                virtual_project: false,
                index: None,
                url: Some(dist.url.to_string()),
                path: Some(dist.install_path.display().to_string()),
                purl: None,
            },
            Dist::Source(SourceDist::Registry(dist)) => Self {
                name: dist.name().to_string(),
                version: Some(dist.version.to_string()),
                distribution_kind: "source",
                source_kind: "registry",
                requires_prepare: true,
                editable: false,
                virtual_project: false,
                index: Some(dist.index.to_string()),
                url: None,
                path: None,
                purl: Some(socket_purl(dist.name(), &dist.version)),
            },
            Dist::Source(SourceDist::DirectUrl(dist)) => Self {
                name: dist.name().to_string(),
                version: None,
                distribution_kind: "source",
                source_kind: "direct-url",
                requires_prepare: true,
                editable: false,
                virtual_project: false,
                index: None,
                url: Some(dist.url.to_string()),
                path: None,
                purl: None,
            },
            Dist::Source(SourceDist::Git(dist)) => Self {
                name: dist.name().to_string(),
                version: None,
                distribution_kind: "source",
                source_kind: "git",
                requires_prepare: true,
                editable: false,
                virtual_project: false,
                index: None,
                url: Some(dist.url.to_string()),
                path: None,
                purl: None,
            },
            Dist::Source(SourceDist::Path(dist)) => Self {
                name: dist.name().to_string(),
                version: dist.version.as_ref().map(ToString::to_string),
                distribution_kind: "source",
                source_kind: "path",
                requires_prepare: true,
                editable: false,
                virtual_project: false,
                index: None,
                url: Some(dist.url.to_string()),
                path: Some(dist.install_path.display().to_string()),
                purl: None,
            },
            Dist::Source(SourceDist::Directory(dist)) => Self {
                name: dist.name().to_string(),
                version: None,
                distribution_kind: "source",
                source_kind: "directory",
                requires_prepare: true,
                editable: dist.editable.unwrap_or(false),
                virtual_project: dist.r#virtual.unwrap_or(false),
                index: None,
                url: Some(dist.url.to_string()),
                path: Some(dist.install_path.display().to_string()),
                purl: None,
            },
        }
    }

    fn from_cached(dist: &CachedDist) -> Self {
        match dist {
            CachedDist::Registry(dist) => Self {
                name: dist.name().to_string(),
                version: Some(dist.filename.version.to_string()),
                distribution_kind: "cached",
                source_kind: "registry",
                requires_prepare: false,
                editable: false,
                virtual_project: false,
                index: None,
                url: None,
                path: Some(dist.path.display().to_string()),
                purl: Some(socket_purl(dist.name(), &dist.filename.version)),
            },
            CachedDist::Url(dist) => {
                let (source_kind, url, path, editable, virtual_project) =
                    package_fields_from_parsed_url(&dist.url.parsed_url);

                Self {
                    name: dist.name().to_string(),
                    version: Some(dist.filename.version.to_string()),
                    distribution_kind: "cached",
                    source_kind,
                    requires_prepare: false,
                    editable,
                    virtual_project,
                    index: None,
                    url,
                    path: path.or_else(|| Some(dist.path.display().to_string())),
                    purl: None,
                }
            }
        }
    }
}

impl DependencyGuardInstalled {
    fn from_installed(dist: &InstalledDist) -> Self {
        Self {
            name: dist.name().to_string(),
            version: dist.version().to_string(),
            path: dist.install_path().display().to_string(),
        }
    }
}

pub(crate) async fn run_dependency_guard(
    settings: &DependencyGuardSettings,
    cached: &[CachedDist],
    remote: &[Arc<Dist>],
    reinstalls: &[InstalledDist],
    extraneous: &[InstalledDist],
) -> anyhow::Result<()> {
    if !settings.enabled() {
        return Ok(());
    }

    let payload = DependencyGuardPayload::from_plan(
        settings.socket_min_score,
        cached,
        remote,
        reinstalls,
        extraneous,
    );

    if payload.is_empty() {
        debug!("Skipping dependency guard because the install plan contains no package changes");
        return Ok(());
    }

    debug!(
        "Running dependency guard with {} provider(s) over {} remote and {} cached package(s)",
        settings.providers.len(),
        payload.remote.len(),
        payload.cached.len(),
    );

    for provider in &settings.providers {
        match provider {
            DependencyGuardProvider::Socket => {
                run_socket_provider(settings.socket_min_score, &payload).await?;
            }
            DependencyGuardProvider::Command => {
                let command = settings.command.as_deref().context(
                    "Dependency guard provider `command` requires `--dependency-guard-command` or `dependency-guard-command` in configuration",
                )?;
                run_command_provider(command, &payload).await?;
            }
        }
    }

    Ok(())
}

async fn run_socket_provider(
    socket_min_score: u8,
    payload: &DependencyGuardPayload,
) -> anyhow::Result<()> {
    let targets = payload.socket_targets();
    if targets.is_empty() {
        debug!("Skipping Socket dependency guard because no registry packages are being installed");
        return Ok(());
    }

    for target in targets {
        let output = match Command::new("socket")
            .arg("package")
            .arg("score")
            .arg("--json")
            .arg(target)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
        {
            Ok(output) => output,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                bail!(
                    "Dependency guard provider `socket` requires the `socket` CLI to be installed and available on PATH"
                );
            }
            Err(err) => {
                return Err(err)
                    .with_context(|| format!("Failed to invoke Socket CLI for `{target}`"));
            }
        };

        if !output.status.success() {
            let detail = render_process_output(&output.stdout, &output.stderr)
                .map(|detail| format!(": {detail}"))
                .unwrap_or_default();
            bail!("Socket dependency guard failed for `{target}`{detail}");
        }

        let value: Value = serde_json::from_slice(&output.stdout)
            .with_context(|| format!("Failed to parse Socket CLI JSON output for `{target}`"))?;
        let score = extract_socket_score(&value).with_context(|| {
            format!("Socket CLI did not return a recognizable score for `{target}`")
        })?;

        if score < f64::from(socket_min_score) {
            bail!(
                "Socket dependency guard blocked `{target}` because the score {} is below the minimum {}",
                format_socket_score(score),
                socket_min_score,
            );
        }
    }

    Ok(())
}

async fn run_command_provider(
    command: &[String],
    payload: &DependencyGuardPayload,
) -> anyhow::Result<()> {
    let (program, args) = command
        .split_first()
        .context("Dependency guard command is empty")?;

    let json = serde_json::to_vec(payload).context("Failed to serialize dependency guard plan")?;
    let mut child = match Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            bail!(
                "Dependency guard command `{}` was not found on PATH",
                display_command(command)
            );
        }
        Err(err) => {
            return Err(err).with_context(|| {
                format!(
                    "Failed to start dependency guard command `{}`",
                    display_command(command)
                )
            });
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(&json).await.with_context(|| {
            format!(
                "Failed to send dependency guard payload to `{}`",
                display_command(command)
            )
        })?;
    }

    let output = child.wait_with_output().await.with_context(|| {
        format!(
            "Failed while waiting for dependency guard command `{}`",
            display_command(command)
        )
    })?;

    if !output.status.success() {
        let detail = render_process_output(&output.stdout, &output.stderr)
            .map(|detail| format!(": {detail}"))
            .unwrap_or_default();
        bail!(
            "Dependency guard command `{}` blocked installation{detail}",
            display_command(command),
        );
    }

    Ok(())
}

fn package_fields_from_parsed_url(
    parsed_url: &ParsedUrl,
) -> (&'static str, Option<String>, Option<String>, bool, bool) {
    match parsed_url {
        ParsedUrl::Archive(url) => ("direct-url", Some(url.url.to_string()), None, false, false),
        ParsedUrl::Git(url) => ("git", Some(url.url.to_string()), None, false, false),
        ParsedUrl::Path(url) => (
            "path",
            Some(url.url.to_string()),
            Some(url.install_path.display().to_string()),
            false,
            false,
        ),
        ParsedUrl::Directory(url) => (
            "directory",
            Some(url.url.to_string()),
            Some(url.install_path.display().to_string()),
            url.editable.unwrap_or(false),
            url.r#virtual.unwrap_or(false),
        ),
    }
}

fn socket_purl(name: &impl std::fmt::Display, version: &impl std::fmt::Display) -> String {
    format!("pkg:pypi/{name}@{version}")
}

fn display_command(command: &[String]) -> String {
    command.join(" ")
}

fn render_process_output(stdout: &[u8], stderr: &[u8]) -> Option<String> {
    let stderr = String::from_utf8_lossy(stderr);
    if let Some(line) = stderr.lines().find(|line| !line.trim().is_empty()) {
        return Some(line.trim().to_string());
    }

    let stdout = String::from_utf8_lossy(stdout);
    stdout
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
}

fn format_socket_score(score: f64) -> String {
    if (score.fract() - 0.0).abs() < f64::EPSILON {
        format!("{score:.0}")
    } else {
        format!("{score:.1}")
    }
}

fn extract_socket_score(value: &Value) -> Option<f64> {
    let mut stack = vec![(Vec::<String>::new(), value)];

    while let Some((path, value)) = stack.pop() {
        match value {
            Value::Number(number) => {
                if let Some(score) = number.as_f64() {
                    if matches!(
                        path.as_slice(),
                        [key] if key == "score" || key == "depScore" || key == "depscore" || key == "overallScore"
                    ) || matches!(
                        path.as_slice(),
                        [parent, child]
                            if (parent == "score" || parent == "scores") && child == "overall"
                    ) {
                        return Some(score);
                    }
                }
            }
            Value::Array(items) => {
                for item in items.iter().rev() {
                    stack.push((path.clone(), item));
                }
            }
            Value::Object(map) => {
                for (key, value) in map {
                    let mut child_path = path.clone();
                    child_path.push(key.clone());
                    stack.push((child_path, value));
                }
            }
            Value::Null | Value::Bool(_) | Value::String(_) => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::extract_socket_score;

    #[test]
    fn extracts_socket_score_from_nested_overall_score() {
        let value = json!({
            "score": {
                "overall": 92
            }
        });

        assert_eq!(extract_socket_score(&value), Some(92.0));
    }

    #[test]
    fn extracts_socket_score_from_dep_score() {
        let value = json!({
            "depScore": 88
        });

        assert_eq!(extract_socket_score(&value), Some(88.0));
    }

    #[test]
    fn ignores_unrecognized_numeric_fields() {
        let value = json!({
            "data": {
                "unexpected": 77
            }
        });

        assert_eq!(extract_socket_score(&value), None);
    }
}
