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
    #[serde(skip_serializing_if = "Option::is_none")]
    import_ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_error: Option<String>,
}

#[derive(Debug, Default, Serialize)]
struct TorchDoctorPackages {
    torch: TorchDoctorPackage,
    torchvision: TorchDoctorPackage,
    torchaudio: TorchDoctorPackage,
}

#[derive(Debug, Default, Serialize)]
struct TorchDoctorRuntime {
    #[serde(skip_serializing_if = "Option::is_none")]
    cuda_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hip_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cuda_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xpu_available: Option<bool>,
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
    torch_runtime: TorchDoctorRuntime,
    next_command: String,
    notes: Vec<String>,
}

#[derive(Debug, Default)]
struct TorchDoctorInspection {
    installed_packages: TorchDoctorPackages,
    torch_runtime: TorchDoctorRuntime,
}

#[derive(Debug, Default, Deserialize)]
struct PythonDoctorPackage {
    version: Option<String>,
    #[serde(default)]
    import_ok: Option<bool>,
    #[serde(default)]
    import_error: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PythonTorchDoctorRuntime {
    #[serde(default)]
    cuda_version: Option<String>,
    #[serde(default)]
    hip_version: Option<String>,
    #[serde(default)]
    cuda_available: Option<bool>,
    #[serde(default)]
    xpu_available: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
struct PythonDoctorInspection {
    #[serde(default)]
    torch: PythonDoctorPackage,
    #[serde(default)]
    torchvision: PythonDoctorPackage,
    #[serde(default)]
    torchaudio: PythonDoctorPackage,
    #[serde(default)]
    torch_runtime: PythonTorchDoctorRuntime,
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

    let inspection = if let Some(environment) = environment.as_ref() {
        match inspect_environment(environment.python_executable()) {
            Ok(inspection) => {
                add_probe_notes(
                    &inspection.installed_packages,
                    &inspection.torch_runtime,
                    &recommended_backend,
                    &mut notes,
                );
                inspection
            }
            Err(err) => {
                notes.push(format!(
                    "Could not inspect installed PyTorch packages: {}.",
                    err.to_string().trim()
                ));
                TorchDoctorInspection::default()
            }
        }
    } else {
        TorchDoctorInspection::default()
    };

    notes.push(
        "Import and runtime checks currently cover `torch`, `torchvision`, and `torchaudio` only."
            .to_string(),
    );
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
        installed_packages: inspection.installed_packages,
        torch_runtime: inspection.torch_runtime,
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
    if !report.torch_runtime.is_empty() {
        writeln!(printer.stdout())?;
        writeln!(printer.stdout(), "torch runtime:")?;
        write_runtime_value_line(
            printer,
            "torch.version.cuda",
            report.torch_runtime.cuda_version.as_deref(),
        )?;
        write_runtime_value_line(
            printer,
            "torch.version.hip",
            report.torch_runtime.hip_version.as_deref(),
        )?;
        write_runtime_flag_line(
            printer,
            "torch.cuda.is_available()",
            report.torch_runtime.cuda_available,
        )?;
        write_runtime_flag_line(
            printer,
            "torch.xpu.is_available()",
            report.torch_runtime.xpu_available,
        )?;
    }
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

impl TorchDoctorRuntime {
    fn is_empty(&self) -> bool {
        self.cuda_version.is_none()
            && self.hip_version.is_none()
            && self.cuda_available.is_none()
            && self.xpu_available.is_none()
    }
}

fn inspect_environment(python: &Path) -> anyhow::Result<TorchDoctorInspection> {
    let script = r#"
import importlib.metadata as metadata
import importlib
import json

def summarize_exception(exc):
    message = " ".join(str(exc).strip().split())
    if message:
        return f"{type(exc).__name__}: {message}"[:240]
    return type(exc).__name__

def package_probe(name):
    try:
        version = metadata.version(name)
    except metadata.PackageNotFoundError:
        return {"version": None, "import_ok": None, "import_error": None}

    try:
        importlib.import_module(name)
    except Exception as exc:
        return {
            "version": version,
            "import_ok": False,
            "import_error": summarize_exception(exc),
        }

    return {"version": version, "import_ok": True, "import_error": None}

packages = {name: package_probe(name) for name in ("torch", "torchvision", "torchaudio")}

runtime = {}
if packages["torch"]["import_ok"]:
    import torch

    runtime["cuda_version"] = getattr(getattr(torch, "version", None), "cuda", None)
    runtime["hip_version"] = getattr(getattr(torch, "version", None), "hip", None)

    if hasattr(torch, "cuda") and hasattr(torch.cuda, "is_available"):
        try:
            runtime["cuda_available"] = bool(torch.cuda.is_available())
        except Exception:
            runtime["cuda_available"] = None
    else:
        runtime["cuda_available"] = None

    if hasattr(torch, "xpu") and hasattr(torch.xpu, "is_available"):
        try:
            runtime["xpu_available"] = bool(torch.xpu.is_available())
        except Exception:
            runtime["xpu_available"] = None
    else:
        runtime["xpu_available"] = None

print(json.dumps({**packages, "torch_runtime": runtime}))
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
    let inspection: PythonDoctorInspection = serde_json::from_str(stdout.trim())?;
    Ok(TorchDoctorInspection {
        installed_packages: TorchDoctorPackages {
            torch: package_from_probe(inspection.torch),
            torchvision: package_from_probe(inspection.torchvision),
            torchaudio: package_from_probe(inspection.torchaudio),
        },
        torch_runtime: TorchDoctorRuntime {
            cuda_version: inspection.torch_runtime.cuda_version,
            hip_version: inspection.torch_runtime.hip_version,
            cuda_available: inspection.torch_runtime.cuda_available,
            xpu_available: inspection.torch_runtime.xpu_available,
        },
    })
}

fn package_from_probe(probe: PythonDoctorPackage) -> TorchDoctorPackage {
    let backend = probe.version.as_deref().and_then(version_backend);
    TorchDoctorPackage {
        version: probe.version,
        backend,
        import_ok: probe.import_ok,
        import_error: probe.import_error.map(trim_diagnostic),
    }
}

fn version_backend(version: &str) -> Option<String> {
    let (_, backend) = version.split_once('+')?;
    TorchBackend::from_str(backend).ok()?;
    Some(backend.to_string())
}

fn trim_diagnostic(message: String) -> String {
    let trimmed = message.trim().to_string();
    if trimmed.len() <= 240 {
        trimmed
    } else {
        format!("{}...", &trimmed[..237])
    }
}

fn add_probe_notes(
    packages: &TorchDoctorPackages,
    runtime: &TorchDoctorRuntime,
    recommended_backend: &str,
    notes: &mut Vec<String>,
) {
    add_import_failure_notes(packages, notes);
    add_backend_mismatch_notes(packages, recommended_backend, notes);
    add_runtime_notes(packages, runtime, notes);
}

fn add_import_failure_notes(packages: &TorchDoctorPackages, notes: &mut Vec<String>) {
    for (name, package) in [
        ("torch", &packages.torch),
        ("torchvision", &packages.torchvision),
        ("torchaudio", &packages.torchaudio),
    ] {
        if package.import_ok == Some(false) {
            if let Some(error) = package.import_error.as_deref() {
                notes.push(format!("Importing {name} failed: {error}."));
            } else {
                notes.push(format!("Importing {name} failed."));
            }
        }
    }
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

fn add_runtime_notes(
    packages: &TorchDoctorPackages,
    runtime: &TorchDoctorRuntime,
    notes: &mut Vec<String>,
) {
    let Some(backend) = packages.torch.backend.as_deref() else {
        return;
    };

    match torch_backend_family(backend) {
        TorchBackendFamily::Cuda => {
            if runtime.cuda_version.is_none() {
                notes.push(format!(
                    "Installed torch reports CUDA backend `{backend}`, but `torch.version.cuda` is unavailable."
                ));
            }
            if runtime.cuda_available == Some(false) {
                notes.push(format!(
                    "Installed torch reports CUDA backend `{backend}`, but `torch.cuda.is_available()` returned false. This may indicate a runtime visibility or compatibility problem."
                ));
            }
        }
        TorchBackendFamily::Rocm => {
            if runtime.hip_version.is_none() {
                notes.push(format!(
                    "Installed torch reports ROCm backend `{backend}`, but `torch.version.hip` is unavailable."
                ));
            }
        }
        TorchBackendFamily::Xpu => {
            if runtime.xpu_available == Some(false) {
                notes.push(
                    "Installed torch reports XPU backend `xpu`, but `torch.xpu.is_available()` returned false. This may indicate an Intel runtime visibility or compatibility problem."
                        .to_string(),
                );
            }
        }
        TorchBackendFamily::Cpu | TorchBackendFamily::Unknown => {}
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TorchBackendFamily {
    Cpu,
    Cuda,
    Rocm,
    Xpu,
    Unknown,
}

fn torch_backend_family(backend: &str) -> TorchBackendFamily {
    if backend == "cpu" {
        TorchBackendFamily::Cpu
    } else if backend == "xpu" {
        TorchBackendFamily::Xpu
    } else if backend.starts_with("cu") {
        TorchBackendFamily::Cuda
    } else if backend.starts_with("rocm") {
        TorchBackendFamily::Rocm
    } else {
        TorchBackendFamily::Unknown
    }
}

fn write_package_line(
    printer: Printer,
    name: &str,
    package: &TorchDoctorPackage,
) -> Result<(), std::fmt::Error> {
    match package.version.as_deref() {
        Some(version) => {
            if package.import_ok == Some(true) {
                writeln!(
                    printer.stdout(),
                    "{name}: {} {}",
                    version.cyan(),
                    "(import ok)".dimmed()
                )
            } else if package.import_ok == Some(false) {
                writeln!(
                    printer.stdout(),
                    "{name}: {} {}",
                    version.cyan(),
                    "(import failed)".yellow()
                )
            } else {
                writeln!(printer.stdout(), "{name}: {}", version.cyan())
            }
        }
        None => writeln!(printer.stdout(), "{name}: {}", "not installed".dimmed()),
    }
}

fn write_runtime_value_line(
    printer: Printer,
    name: &str,
    value: Option<&str>,
) -> Result<(), std::fmt::Error> {
    match value {
        Some(value) => writeln!(printer.stdout(), "{name}: {}", value.cyan()),
        None => writeln!(printer.stdout(), "{name}: {}", "unknown".dimmed()),
    }
}

fn write_runtime_flag_line(
    printer: Printer,
    name: &str,
    value: Option<bool>,
) -> Result<(), std::fmt::Error> {
    match value {
        Some(true) => writeln!(printer.stdout(), "{name}: {}", "true".cyan()),
        Some(false) => writeln!(printer.stdout(), "{name}: {}", "false".cyan()),
        None => writeln!(printer.stdout(), "{name}: {}", "unknown".dimmed()),
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
    use super::{
        TorchDoctorPackage, TorchDoctorPackages, TorchDoctorRuntime, add_probe_notes,
        recommended_backend,
    };
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

    #[test]
    fn probe_notes_report_import_failures() {
        let packages = TorchDoctorPackages {
            torch: TorchDoctorPackage {
                version: Some("2.6.0+cpu".to_string()),
                backend: Some("cpu".to_string()),
                import_ok: Some(false),
                import_error: Some("ImportError: broken torch".to_string()),
            },
            ..TorchDoctorPackages::default()
        };

        let mut notes = Vec::new();
        add_probe_notes(&packages, &TorchDoctorRuntime::default(), "cpu", &mut notes);

        assert!(
            notes
                .iter()
                .any(|note| note.contains("Importing torch failed"))
        );
        assert!(notes.iter().any(|note| note.contains("broken torch")));
    }

    #[test]
    fn probe_notes_report_cuda_runtime_symptoms() {
        let packages = TorchDoctorPackages {
            torch: TorchDoctorPackage {
                version: Some("2.6.0+cu129".to_string()),
                backend: Some("cu129".to_string()),
                import_ok: Some(true),
                import_error: None,
            },
            ..TorchDoctorPackages::default()
        };
        let runtime = TorchDoctorRuntime {
            cuda_version: Some("12.9".to_string()),
            hip_version: None,
            cuda_available: Some(false),
            xpu_available: None,
        };

        let mut notes = Vec::new();
        add_probe_notes(&packages, &runtime, "cu129", &mut notes);

        assert!(
            notes
                .iter()
                .any(|note| note.contains("torch.cuda.is_available()") && note.contains("false"))
        );
    }

    #[test]
    fn probe_notes_report_rocm_runtime_symptoms() {
        let packages = TorchDoctorPackages {
            torch: TorchDoctorPackage {
                version: Some("2.6.0+rocm7.1".to_string()),
                backend: Some("rocm7.1".to_string()),
                import_ok: Some(true),
                import_error: None,
            },
            ..TorchDoctorPackages::default()
        };

        let mut notes = Vec::new();
        add_probe_notes(
            &packages,
            &TorchDoctorRuntime::default(),
            "rocm7.1",
            &mut notes,
        );

        assert!(
            notes
                .iter()
                .any(|note| note.contains("torch.version.hip") && note.contains("unavailable"))
        );
    }
}
