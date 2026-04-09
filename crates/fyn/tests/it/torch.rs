use std::fs;

use anyhow::Result;
use serde_json::Value;

use fyn_static::EnvVars;

fn write_fake_package(
    context: &fyn_test::TestContext,
    name: &str,
    version: &str,
    module_contents: &str,
) -> Result<()> {
    let package_dir = context.site_packages().join(name);
    fs::create_dir_all(&package_dir)?;
    fs::write(package_dir.join("__init__.py"), module_contents)?;

    let dist_info = context
        .site_packages()
        .join(format!("{name}-{version}.dist-info"));
    fs::create_dir_all(&dist_info)?;
    fs::write(
        dist_info.join("METADATA"),
        format!("Metadata-Version: 2.1\nName: {name}\nVersion: {version}\n"),
    )?;

    Ok(())
}

#[test]
fn torch_doctor_json_cuda_override() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let output = context
        .command()
        .arg("torch")
        .arg("doctor")
        .arg("--json")
        .env(EnvVars::UV_CUDA_DRIVER_VERSION, "525.60.13")
        .env_remove(EnvVars::UV_AMD_GPU_ARCHITECTURE)
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    let environment_path = context.venv.path().display().to_string();
    assert_eq!(
        report["environment"]["path"].as_str(),
        Some(environment_path.as_str())
    );
    assert_eq!(report["accelerator"]["kind"].as_str(), Some("cuda"));
    assert_eq!(
        report["accelerator"]["driver_version"].as_str(),
        Some("525.60.13")
    );

    let expected_backend = if cfg!(target_os = "linux") {
        "cu129"
    } else if cfg!(windows) {
        "cu118"
    } else {
        "cpu"
    };
    assert_eq!(
        report["recommended_backend"].as_str(),
        Some(expected_backend)
    );
    let expected_command =
        format!("fyn pip install torch torchvision torchaudio --torch-backend={expected_backend}");
    assert_eq!(
        report["next_command"].as_str(),
        Some(expected_command.as_str())
    );

    let notes = report["notes"].as_array().expect("notes array");
    assert!(notes.iter().any(|note| {
        note.as_str() == Some("Accelerator detection was overridden by `UV_CUDA_DRIVER_VERSION`.")
    }));

    Ok(())
}

#[test]
fn torch_doctor_json_no_packages_installed() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let output = context
        .command()
        .arg("torch")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert!(report["installed_packages"]["torch"]["version"].is_null());
    assert!(report["installed_packages"]["torch"]["import_ok"].is_null());
    assert_eq!(report["torch_runtime"], serde_json::json!({}));

    Ok(())
}

#[test]
fn torch_doctor_json_reports_import_failure_with_metadata() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    write_fake_package(
        &context,
        "torch",
        "2.6.0+cpu",
        r#"raise ImportError("broken torch runtime")"#,
    )?;

    let output = context
        .command()
        .arg("torch")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        report["installed_packages"]["torch"]["version"].as_str(),
        Some("2.6.0+cpu")
    );
    assert_eq!(
        report["installed_packages"]["torch"]["import_ok"].as_bool(),
        Some(false)
    );
    assert!(
        report["installed_packages"]["torch"]["import_error"]
            .as_str()
            .expect("import error")
            .contains("broken torch runtime")
    );
    assert!(
        report["torch_runtime"]
            .as_object()
            .expect("runtime object")
            .is_empty()
    );
    assert!(
        report["notes"]
            .as_array()
            .expect("notes")
            .iter()
            .any(|note| {
                note.as_str()
                    .is_some_and(|note| note.contains("Importing torch failed"))
            })
    );

    Ok(())
}

#[test]
fn torch_doctor_json_reports_runtime_from_fake_torch() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    write_fake_package(
        &context,
        "torch",
        "2.6.0+cpu",
        r#"
__version__ = "2.6.0+cpu"

class _Version:
    cuda = None
    hip = None

version = _Version()

class _Cuda:
    @staticmethod
    def is_available():
        return False

cuda = _Cuda()

class _Xpu:
    @staticmethod
    def is_available():
        return False

xpu = _Xpu()
"#,
    )?;
    write_fake_package(
        &context,
        "torchvision",
        "0.21.0+cpu",
        "__version__ = '0.21.0+cpu'\n",
    )?;
    write_fake_package(
        &context,
        "torchaudio",
        "2.6.0+cpu",
        "__version__ = '2.6.0+cpu'\n",
    )?;

    let output = context
        .command()
        .arg("torch")
        .arg("doctor")
        .arg("--json")
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(
        report["installed_packages"]["torch"]["import_ok"].as_bool(),
        Some(true)
    );
    assert_eq!(
        report["installed_packages"]["torchvision"]["import_ok"].as_bool(),
        Some(true)
    );
    assert_eq!(
        report["installed_packages"]["torchaudio"]["import_ok"].as_bool(),
        Some(true)
    );
    assert_eq!(
        report["torch_runtime"]["cuda_available"].as_bool(),
        Some(false)
    );
    assert_eq!(
        report["torch_runtime"]["xpu_available"].as_bool(),
        Some(false)
    );
    assert!(report["torch_runtime"]["cuda_version"].is_null());
    assert!(report["torch_runtime"]["hip_version"].is_null());

    Ok(())
}

#[test]
fn torch_doctor_json_amd_override() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let output = context
        .command()
        .arg("torch")
        .arg("doctor")
        .arg("--json")
        .env(EnvVars::UV_AMD_GPU_ARCHITECTURE, "gfx1100")
        .env_remove(EnvVars::UV_CUDA_DRIVER_VERSION)
        .output()?;

    assert!(output.status.success());

    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(report["accelerator"]["kind"].as_str(), Some("amd"));
    assert_eq!(
        report["accelerator"]["gpu_architecture"].as_str(),
        Some("gfx1100")
    );

    let expected_backend = if cfg!(target_os = "linux") {
        "rocm7.1"
    } else {
        "cpu"
    };
    assert_eq!(
        report["recommended_backend"].as_str(),
        Some(expected_backend)
    );
    assert!(
        report["reason"]
            .as_str()
            .expect("reason")
            .contains("AMD GPU architecture gfx1100")
    );

    Ok(())
}
