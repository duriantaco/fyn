use anyhow::Result;
use serde_json::Value;

use fyn_static::EnvVars;

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
