use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use assert_fs::fixture::PathChild;
use assert_fs::prelude::*;
use indoc::indoc;

use fyn_test::{copy_dir_ignore, fyn_snapshot};

/// Test basic metadata output for a simple workspace with one member.
#[test]
fn workspace_metadata_simple() {
    let context = fyn_test::test_context!("3.12");

    // Initialize a workspace with one member
    context.init().arg("foo").assert().success();

    let workspace = context.temp_dir.child("foo");

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/foo",
      "members": [
        {
          "name": "foo",
          "path": "[TEMP_DIR]/foo"
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );
}

/// Test metadata for a root workspace (workspace with a root package).
#[test]
fn workspace_metadata_root_workspace() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        context
            .workspace_root
            .join("test/workspaces/albatross-root-workspace"),
        &workspace,
    )?;

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/workspace",
      "members": [
        {
          "name": "albatross",
          "path": "[TEMP_DIR]/workspace"
        },
        {
          "name": "bird-feeder",
          "path": "[TEMP_DIR]/workspace/packages/bird-feeder"
        },
        {
          "name": "seeds",
          "path": "[TEMP_DIR]/workspace/packages/seeds"
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );

    Ok(())
}

/// Test metadata for a virtual workspace (no root package).
#[test]
fn workspace_metadata_virtual_workspace() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        context
            .workspace_root
            .join("test/workspaces/albatross-virtual-workspace"),
        &workspace,
    )?;

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/workspace",
      "members": [
        {
          "name": "albatross",
          "path": "[TEMP_DIR]/workspace/packages/albatross"
        },
        {
          "name": "bird-feeder",
          "path": "[TEMP_DIR]/workspace/packages/bird-feeder"
        },
        {
          "name": "seeds",
          "path": "[TEMP_DIR]/workspace/packages/seeds"
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );

    Ok(())
}

/// Test metadata when run from a workspace member directory.
#[test]
fn workspace_metadata_from_member() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        context
            .workspace_root
            .join("test/workspaces/albatross-root-workspace"),
        &workspace,
    )?;

    let member_dir = workspace.join("packages").join("bird-feeder");

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&member_dir), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/workspace",
      "members": [
        {
          "name": "albatross",
          "path": "[TEMP_DIR]/workspace"
        },
        {
          "name": "bird-feeder",
          "path": "[TEMP_DIR]/workspace/packages/bird-feeder"
        },
        {
          "name": "seeds",
          "path": "[TEMP_DIR]/workspace/packages/seeds"
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );

    Ok(())
}

/// Test metadata for a workspace with multiple packages.
#[test]
fn workspace_metadata_multiple_members() {
    let context = fyn_test::test_context!("3.12");

    // Initialize workspace root
    context.init().arg("pkg-a").assert().success();

    let workspace_root = context.temp_dir.child("pkg-a");

    // Add more members
    context
        .init()
        .arg("pkg-b")
        .current_dir(&workspace_root)
        .assert()
        .success();

    context
        .init()
        .arg("pkg-c")
        .current_dir(&workspace_root)
        .assert()
        .success();

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace_root), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/pkg-a",
      "members": [
        {
          "name": "pkg-a",
          "path": "[TEMP_DIR]/pkg-a"
        },
        {
          "name": "pkg-b",
          "path": "[TEMP_DIR]/pkg-a/pkg-b"
        },
        {
          "name": "pkg-c",
          "path": "[TEMP_DIR]/pkg-a/pkg-c"
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );
}

/// Test metadata for a single project (not a workspace).
#[test]
fn workspace_metadata_single_project() {
    let context = fyn_test::test_context!("3.12");

    context.init().arg("my-project").assert().success();

    let project = context.temp_dir.child("my-project");

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&project), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/my-project",
      "members": [
        {
          "name": "my-project",
          "path": "[TEMP_DIR]/my-project"
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );
}

/// Test metadata with excluded packages.
#[test]
fn workspace_metadata_with_excluded() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let workspace = context.temp_dir.child("workspace");

    copy_dir_ignore(
        context
            .workspace_root
            .join("test/workspaces/albatross-project-in-excluded"),
        &workspace,
    )?;

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/workspace",
      "members": [
        {
          "name": "albatross",
          "path": "[TEMP_DIR]/workspace"
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );

    Ok(())
}

/// Test metadata error when not in a project.
#[test]
fn workspace_metadata_no_project() {
    let context = fyn_test::test_context!("3.12");

    fyn_snapshot!(context.filters(), context.workspace_metadata(), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    error: No `pyproject.toml` found in current directory or any parent directory
    "
    );
}

#[test]
fn workspace_metadata_includes_member_dependencies_from_lock() -> Result<()> {
    let context = fyn_test::test_context!("3.12");
    let workspace = context.temp_dir.child("workspace");

    workspace.child("pyproject.toml").write_str(indoc! {r#"
        [project]
        name = "root"
        version = "0.1.0"
        dependencies = ["member"]

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [tool.fyn.sources]
        member = { workspace = true }

        [tool.fyn.workspace]
        members = ["packages/*"]
    "#})?;
    workspace
        .child("src")
        .child("root")
        .child("__init__.py")
        .touch()?;

    let member = workspace.child("packages").child("member");
    member.child("pyproject.toml").write_str(indoc! {r#"
        [project]
        name = "member"
        version = "0.2.0"

        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"
    "#})?;
    member
        .child("src")
        .child("member")
        .child("__init__.py")
        .touch()?;

    context.lock().current_dir(&workspace).assert().success();

    fyn_snapshot!(context.filters(), context.workspace_metadata().current_dir(&workspace), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    {
      "schema": {
        "version": "preview"
      },
      "workspace_root": "[TEMP_DIR]/workspace",
      "members": [
        {
          "name": "member",
          "path": "[TEMP_DIR]/workspace/packages/member",
          "dependencies": []
        },
        {
          "name": "root",
          "path": "[TEMP_DIR]/workspace",
          "dependencies": [
            "member"
          ]
        }
      ]
    }

    ----- stderr -----
    warning: The `fyn workspace metadata` command is experimental and may change without warning. Pass `--preview-features workspace-metadata` to disable this warning.
    "#
    );

    Ok(())
}
