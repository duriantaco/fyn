use std::fmt::Write;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use anyhow::Context;
use fyn_cache::Cache;
use fyn_fs::Simplified;
use fyn_platform::Libc;
use fyn_platform_tags::{Arch, Os, Platform};
use fyn_preview::Preview;
use fyn_python::{EnvironmentPreference, PythonEnvironment, PythonPreference, PythonRequest};
use fyn_static::EnvVars;
use fyn_torch::{Accelerator, TorchBackend, TorchSource, TorchStrategy};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::commands::ExitStatus;
use crate::printer::Printer;

#[derive(Debug, Serialize)]
struct TorchDoctorEnvironment {
    path: String,
    python: String,
    version: String,
}

#[derive(Debug, Default, Serialize)]
struct TorchDoctorPackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend: Option<String>,
}

#[derive(Debug, Default, Serialize)]
struct TorchDoctorPackages {
    torch: TorchDoctorPackage,
    torchvision: TorchDoctorPackage,
    torchaudio: TorchDoctorPackage,
}

#[derive(Debug, Serialize)]
struct TorchDoctorAccelerator {
    kind: &'static str,
    display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    driver_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gpu_architecture: Option<String>,
}

#[derive(Debug, Serialize)]
struct TorchDoctorReport {
    environment: Option<TorchDoctorEnvironment>,
    platform: String,
    accelerator: TorchDoctorAccelerator,
    recommended_backend: String,
    reason: String,
    installed_packages: TorchDoctorPackages,
    next_command: String,
    notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PythonPackageVersions {
    torch: Option<String>,
    torchvision: Option<String>,
    torchaudio: Option<String>,
}

pub(crate) async fn doctor(
    json: bool,
    python_preference: PythonPreference,
    cache: &Cache,
    preview: Preview,
    printer: Printer,
) -> anyhow::Result<ExitStatus> {
    let environment = PythonEnvironment::find(
        &PythonRequest::default(),
        EnvironmentPreference::from_system_flag(false, false),
        python_preference,
        cache,
        preview,
    )
    .ok();

    let platform = environment
        .as_ref()
        .map(|environment| environment.interpreter().platform().clone())
        .unwrap_or_else(host_platform);
    let accelerator = Accelerator::detect()?;
    let strategy = strategy_from_accelerator(&platform.os().clone(), accelerator.clone());
    let recommended_backend = recommended_backend(&strategy)?;
    let reason = recommendation_reason(&platform, accelerator.as_ref(), &recommended_backend);
    let next_command = format!(
        "fyn pip install torch torchvision torchaudio --torch-backend={recommended_backend}"
    );

    let mut notes = Vec::new();
    if environment.is_none() {
        notes.push(
            "No Python environment was discovered; the recommendation is based on the host machine."
                .to_string(),
        );
    }
    if std::env::var_os(EnvVars::UV_CUDA_DRIVER_VERSION).is_some() {
        notes.push("Accelerator detection was overridden by `UV_CUDA_DRIVER_VERSION`.".to_string());
    }
    if std::env::var_os(EnvVars::UV_AMD_GPU_ARCHITECTURE).is_some() {
        notes
            .push("Accelerator detection was overridden by `UV_AMD_GPU_ARCHITECTURE`.".to_string());
    }

    let installed_packages = if let Some(environment) = environment.as_ref() {
        match inspect_installed_packages(environment.python_executable()) {
            Ok(packages) => {
                add_backend_mismatch_notes(&packages, &recommended_backend, &mut notes);
                packages
            }
            Err(err) => {
                notes.push(format!(
                    "Could not inspect installed PyTorch packages: {}.",
                    err.to_string().trim()
                ));
                TorchDoctorPackages::default()
            }
        }
    } else {
        TorchDoctorPackages::default()
    };

    notes.push("`fyn torch doctor` only reports the recommended backend.".to_string());
    notes.push("Recreate the environment when switching GPU or backend families.".to_string());

    let report = TorchDoctorReport {
        environment: environment
            .as_ref()
            .map(|environment| TorchDoctorEnvironment {
                path: environment.root().simplified_display().to_string(),
                python: environment
                    .python_executable()
                    .simplified_display()
                    .to_string(),
                version: environment.interpreter().python_full_version().to_string(),
            }),
        platform: platform.pretty(),
        accelerator: accelerator_report(accelerator.as_ref()),
        recommended_backend: recommended_backend.clone(),
        reason,
        installed_packages,
        next_command,
        notes,
    };

    if json {
        writeln!(
            printer.stdout(),
            "{}",
            serde_json::to_string_pretty(&report)?
        )?;
        return Ok(ExitStatus::Success);
    }

    writeln!(printer.stdout(), "{}", "PyTorch doctor".bold())?;
    if let Some(environment) = report.environment.as_ref() {
        writeln!(printer.stdout(), "environment: {}", environment.path.cyan())?;
        writeln!(
            printer.stdout(),
            "python: {} ({})",
            environment.python.cyan(),
            environment.version.cyan()
        )?;
    } else {
        writeln!(printer.stdout(), "environment: {}", "not found".dimmed())?;
        writeln!(printer.stdout(), "python: {}", "not found".dimmed())?;
    }
    writeln!(printer.stdout(), "platform: {}", report.platform.cyan())?;
    writeln!(
        printer.stdout(),
        "accelerator: {}",
        report.accelerator.display.cyan()
    )?;
    writeln!(
        printer.stdout(),
        "recommended backend: {}",
        report.recommended_backend.cyan()
    )?;
    writeln!(printer.stdout(), "reason: {}", report.reason)?;
    writeln!(printer.stdout())?;
    writeln!(printer.stdout(), "installed packages:")?;
    write_package_line(printer, "torch", &report.installed_packages.torch)?;
    write_package_line(
        printer,
        "torchvision",
        &report.installed_packages.torchvision,
    )?;
    write_package_line(printer, "torchaudio", &report.installed_packages.torchaudio)?;
    writeln!(printer.stdout())?;
    writeln!(printer.stdout(), "next command:")?;
    writeln!(printer.stdout(), "  {}", report.next_command.cyan())?;
    if !report.notes.is_empty() {
        writeln!(printer.stdout())?;
        writeln!(printer.stdout(), "notes:")?;
        for note in &report.notes {
            writeln!(printer.stdout(), "- {note}")?;
        }
    }

    Ok(ExitStatus::Success)
}

fn strategy_from_accelerator(os: &Os, accelerator: Option<Accelerator>) -> TorchStrategy {
    match accelerator {
        Some(Accelerator::Cuda { driver_version }) => TorchStrategy::Cuda {
            os: os.clone(),
            driver_version,
            source: TorchSource::PyTorch,
        },
        Some(Accelerator::Amd { gpu_architecture }) => TorchStrategy::Amd {
            os: os.clone(),
            gpu_architecture,
            source: TorchSource::PyTorch,
        },
        Some(Accelerator::Xpu) => TorchStrategy::Xpu {
            os: os.clone(),
            source: TorchSource::PyTorch,
        },
        None => TorchStrategy::Backend {
            backend: TorchBackend::Cpu,
            source: TorchSource::PyTorch,
        },
    }
}

fn recommended_backend(strategy: &TorchStrategy) -> anyhow::Result<String> {
    let index = strategy
        .index_urls()
        .next()
        .context("No compatible PyTorch backend was found for this machine")?;
    let backend = index
        .url()
        .path_segments()
        .and_then(|mut segments| segments.rfind(|segment| !segment.is_empty()))
        .context("Failed to extract the recommended PyTorch backend from the resolved index URL")?;
    Ok(backend.to_string())
}

fn recommendation_reason(
    platform: &Platform,
    accelerator: Option<&Accelerator>,
    recommended_backend: &str,
) -> String {
    match accelerator {
        Some(Accelerator::Cuda { driver_version }) if recommended_backend == "cpu" => format!(
            "Detected NVIDIA driver {driver_version}, but no compatible CUDA backend is available for {}.",
            platform.pretty()
        ),
        Some(Accelerator::Cuda { driver_version }) => format!(
            "Detected NVIDIA driver {driver_version}; selected the highest compatible CUDA backend for {}.",
            platform.pretty()
        ),
        Some(Accelerator::Amd { gpu_architecture }) if recommended_backend == "cpu" => format!(
            "Detected AMD GPU architecture {gpu_architecture}, but no matching ROCm backend is available for {}.",
            platform.pretty()
        ),
        Some(Accelerator::Amd { gpu_architecture }) => format!(
            "Detected AMD GPU architecture {gpu_architecture}; selected the matching ROCm backend."
        ),
        Some(Accelerator::Xpu) if recommended_backend == "xpu" => {
            "Detected an Intel GPU; selected the XPU backend.".to_string()
        }
        Some(Accelerator::Xpu) => format!(
            "Detected an Intel GPU, but XPU wheels are not available for {}.",
            platform.pretty()
        ),
        None => "No supported GPU accelerator was detected; using CPU wheels.".to_string(),
    }
}

fn accelerator_report(accelerator: Option<&Accelerator>) -> TorchDoctorAccelerator {
    match accelerator {
        Some(Accelerator::Cuda { driver_version }) => TorchDoctorAccelerator {
            kind: "cuda",
            display: format!("CUDA {driver_version}"),
            driver_version: Some(driver_version.to_string()),
            gpu_architecture: None,
        },
        Some(Accelerator::Amd { gpu_architecture }) => TorchDoctorAccelerator {
            kind: "amd",
            display: format!("AMD {gpu_architecture}"),
            driver_version: None,
            gpu_architecture: Some(gpu_architecture.to_string()),
        },
        Some(Accelerator::Xpu) => TorchDoctorAccelerator {
            kind: "xpu",
            display: "Intel GPU (XPU)".to_string(),
            driver_version: None,
            gpu_architecture: None,
        },
        None => TorchDoctorAccelerator {
            kind: "none",
            display: "none detected".to_string(),
            driver_version: None,
            gpu_architecture: None,
        },
    }
}

fn inspect_installed_packages(python: &Path) -> anyhow::Result<TorchDoctorPackages> {
    let script = r#"
import importlib.metadata as metadata
import json

packages = {}
for name in ("torch", "torchvision", "torchaudio"):
    try:
        packages[name] = metadata.version(name)
    except metadata.PackageNotFoundError:
        packages[name] = None

print(json.dumps(packages))
"#;

    let output = Command::new(python)
        .arg("-I")
        .arg("-c")
        .arg(script)
        .output()
        .with_context(|| {
            format!(
                "failed to run `{}` to inspect installed packages",
                python.display()
            )
        })?;

    if !output.status.success() {
        anyhow::bail!(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let versions: PythonPackageVersions = serde_json::from_str(stdout.trim())?;
    Ok(TorchDoctorPackages {
        torch: package_from_version(versions.torch),
        torchvision: package_from_version(versions.torchvision),
        torchaudio: package_from_version(versions.torchaudio),
    })
}

fn package_from_version(version: Option<String>) -> TorchDoctorPackage {
    let backend = version.as_deref().and_then(version_backend);
    TorchDoctorPackage { version, backend }
}

fn version_backend(version: &str) -> Option<String> {
    let (_, backend) = version.split_once('+')?;
    TorchBackend::from_str(backend).ok()?;
    Some(backend.to_string())
}

fn add_backend_mismatch_notes(
    packages: &TorchDoctorPackages,
    recommended_backend: &str,
    notes: &mut Vec<String>,
) {
    let backends = [
        ("torch", packages.torch.backend.as_deref()),
        ("torchvision", packages.torchvision.backend.as_deref()),
        ("torchaudio", packages.torchaudio.backend.as_deref()),
    ];

    let mut detected = backends
        .into_iter()
        .filter_map(|(name, backend)| backend.map(|backend| (name, backend)))
        .collect::<Vec<_>>();

    if detected.is_empty() {
        return;
    }

    detected.sort_unstable_by_key(|(name, _)| *name);

    let distinct = detected
        .iter()
        .map(|(_, backend)| *backend)
        .collect::<std::collections::BTreeSet<_>>();
    if distinct.len() > 1 {
        notes.push(format!(
            "Installed PyTorch packages report multiple backend suffixes: {}.",
            detected
                .iter()
                .map(|(name, backend)| format!("{name}={backend}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    } else if let Some((name, backend)) = detected.first()
        && *backend != recommended_backend
    {
        notes.push(format!(
            "Installed {name} reports backend `{backend}`, but this machine recommends `{recommended_backend}`."
        ));
    }
}

fn write_package_line(
    printer: Printer,
    name: &str,
    package: &TorchDoctorPackage,
) -> Result<(), std::fmt::Error> {
    match package.version.as_deref() {
        Some(version) => writeln!(printer.stdout(), "{name}: {}", version.cyan()),
        None => writeln!(printer.stdout(), "{name}: {}", "not installed".dimmed()),
    }
}

fn host_platform() -> Platform {
    let arch = Arch::from_str(std::env::consts::ARCH).unwrap_or(Arch::X86_64);
    let os = match std::env::consts::OS {
        "linux" => match Libc::from_env() {
            Ok(libc) if libc.is_musl() => Os::Musllinux { major: 1, minor: 2 },
            _ => Os::Manylinux {
                major: 2,
                minor: 28,
            },
        },
        "macos" => Os::Macos {
            major: 13,
            minor: 0,
        },
        "windows" => Os::Windows,
        "freebsd" => Os::FreeBsd {
            release: "unknown".to_string(),
        },
        "netbsd" => Os::NetBsd {
            release: "unknown".to_string(),
        },
        "openbsd" => Os::OpenBsd {
            release: "unknown".to_string(),
        },
        "dragonfly" => Os::Dragonfly {
            release: "unknown".to_string(),
        },
        "illumos" => Os::Illumos {
            release: "unknown".to_string(),
            arch: arch.to_string(),
        },
        "haiku" => Os::Haiku {
            release: "unknown".to_string(),
        },
        "android" => Os::Android { api_level: 0 },
        "ios" => Os::Ios {
            major: 0,
            minor: 0,
            simulator: false,
        },
        _ => Os::Manylinux {
            major: 2,
            minor: 28,
        },
    };
    Platform::new(os, arch)
}

#[cfg(test)]
mod tests {
    use super::recommended_backend;
    use fyn_torch::{TorchBackend, TorchSource, TorchStrategy};

    #[test]
    fn recommended_backend_extracts_trailing_segment() {
        let strategy = TorchStrategy::Backend {
            backend: TorchBackend::Cu130,
            source: TorchSource::PyTorch,
        };

        assert_eq!(recommended_backend(&strategy).unwrap(), "cu130");
    }

    #[test]
    fn recommended_backend_handles_rocm_backend_names() {
        let strategy = TorchStrategy::Backend {
            backend: TorchBackend::Rocm71,
            source: TorchSource::PyTorch,
        };

        assert_eq!(recommended_backend(&strategy).unwrap(), "rocm7.1");
    }
}
