use assert_cmd::assert::OutputAssertExt;
use assert_fs::prelude::*;
use indoc::indoc;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use fyn_test::fyn_snapshot;

#[tokio::test]
async fn audit_service_url_override() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": []}]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_non_existent_extra() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]

        [project.optional-dependencies]
        types = ["sniffio==1.3.1"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": []}]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--extra")
        .arg("baz")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Extra `baz` is not defined in any project's `optional-dependencies` table
    ");
}

#[tokio::test]
async fn audit_non_existent_group() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]

        [dependency-groups]
        lint = ["sniffio==1.3.1"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": []}]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--group")
        .arg("baz")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Group `baz` is not defined in any project's `dependency-groups` table
    ");
}

#[tokio::test]
async fn audit_ignore_by_id() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": [{"id": "PYSEC-2023-0001"}]}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/vulns/PYSEC-2023-0001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "PYSEC-2023-0001",
            "modified": "2026-01-01T00:00:00Z",
            "summary": "A test vulnerability in iniconfig",
            "affected": [{
                "ranges": [{
                    "type": "ECOSYSTEM",
                    "events": [
                        {"introduced": "0"},
                        {"fixed": "2.1.0"}
                    ]
                }]
            }]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: false
    exit_code: 1
    ----- stdout -----

    Vulnerabilities:

    iniconfig 2.0.0 has 1 known vulnerability:

    - PYSEC-2023-0001: A test vulnerability in iniconfig

      Fixed in: 2.1.0

      Advisory information: https://osv.dev/vulnerability/PYSEC-2023-0001


    ----- stderr -----
    Found 1 known vulnerability and no adverse project statuses in 1 package
    ");

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--ignore")
        .arg("PYSEC-2023-0001")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_by_alias() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": [{"id": "OSV-2023-0001"}]}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/vulns/OSV-2023-0001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "OSV-2023-0001",
            "modified": "2026-01-01T00:00:00Z",
            "summary": "A vulnerability with aliases",
            "aliases": ["PYSEC-2023-0042", "CVE-2023-9999"]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--ignore")
        .arg("CVE-2023-9999")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_until_fixed() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": [{"id": "VULN-NO-FIX"}]}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/vulns/VULN-NO-FIX"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "VULN-NO-FIX",
            "modified": "2026-01-01T00:00:00Z",
            "summary": "A vulnerability with no fix available"
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--ignore-until-fixed")
        .arg("VULN-NO-FIX")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_until_fixed_with_fix() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": [{"id": "PYSEC-2023-0001"}]}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/vulns/PYSEC-2023-0001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "PYSEC-2023-0001",
            "modified": "2026-01-01T00:00:00Z",
            "summary": "A test vulnerability in iniconfig",
            "affected": [{
                "ranges": [{
                    "type": "ECOSYSTEM",
                    "events": [
                        {"introduced": "0"},
                        {"fixed": "2.1.0"}
                    ]
                }]
            }]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--ignore-until-fixed")
        .arg("PYSEC-2023-0001")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: false
    exit_code: 1
    ----- stdout -----

    Vulnerabilities:

    iniconfig 2.0.0 has 1 known vulnerability:

    - PYSEC-2023-0001: A test vulnerability in iniconfig

      Fixed in: 2.1.0

      Advisory information: https://osv.dev/vulnerability/PYSEC-2023-0001


    ----- stderr -----
    Found 1 known vulnerability and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_config() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]

        [tool.fyn.audit]
        ignore = ["PYSEC-2023-0001"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": [{"id": "PYSEC-2023-0001"}]}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/vulns/PYSEC-2023-0001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "PYSEC-2023-0001",
            "modified": "2026-01-01T00:00:00Z",
            "summary": "A test vulnerability in iniconfig",
            "affected": [{
                "ranges": [{
                    "type": "ECOSYSTEM",
                    "events": [
                        {"introduced": "0"},
                        {"fixed": "2.1.0"}
                    ]
                }]
            }]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_until_fixed_config() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]

        [tool.fyn.audit]
        ignore-until-fixed = ["VULN-NO-FIX"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": [{"id": "VULN-NO-FIX"}]}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/vulns/VULN-NO-FIX"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "VULN-NO-FIX",
            "modified": "2026-01-01T00:00:00Z",
            "summary": "A vulnerability with no fix available"
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_unmatched() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": []}]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--ignore")
        .arg("CVE-XXXX-YYYY")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: Ignored vulnerability `CVE-XXXX-YYYY` does not match any vulnerability in the project
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_until_fixed_unmatched() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": []}]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--ignore-until-fixed")
        .arg("CVE-XXXX-YYYY")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: Ignored vulnerability `CVE-XXXX-YYYY` does not match any vulnerability in the project
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_ignore_mixed_matched_unmatched() {
    let context = fyn_test::test_context!("3.12");

    let pyproject_toml = context.temp_dir.child("pyproject.toml");
    pyproject_toml
        .write_str(indoc! {r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig==2.0.0"]
    "#})
        .unwrap();

    context.lock().assert().success();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": [{"id": "PYSEC-2023-0001"}]}]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/vulns/PYSEC-2023-0001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "PYSEC-2023-0001",
            "modified": "2026-01-01T00:00:00Z",
            "summary": "A test vulnerability in iniconfig",
            "affected": [{
                "ranges": [{
                    "type": "ECOSYSTEM",
                    "events": [
                        {"introduced": "0"},
                        {"fixed": "2.1.0"}
                    ]
                }]
            }]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--ignore")
        .arg("PYSEC-2023-0001")
        .arg("--ignore")
        .arg("CVE-DOES-NOT-EXIST")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warning: Ignored vulnerability `CVE-DOES-NOT-EXIST` does not match any vulnerability in the project
    Found no known vulnerabilities and no adverse project statuses in 1 package
    ");
}
