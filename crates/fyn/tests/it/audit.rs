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

#[tokio::test]
async fn audit_script_no_vulnerabilities() {
    let context = fyn_test::test_context!("3.12");

    let script = context.temp_dir.child("script.py");
    script
        .write_str(indoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = [
        #   "iniconfig==2.0.0",
        # ]
        # ///
        import iniconfig
    "#})
        .unwrap();

    let lockfile = context.temp_dir.child("script.py.lock");
    lockfile
        .write_str(indoc! {r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [manifest]
        requirements = [{ name = "iniconfig", specifier = "==2.0.0" }]

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3", size = 4646, upload-time = "2023-01-07T12:52:09.585Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beebd3ce04b132a8bf3491/iniconfig-2.0.0-py3-none-any.whl", hash = "sha256:b6a85871a79d2e3b22d2d1b94ac2824226a63c6b741c88f7ae975f18b6778374", size = 5892, upload-time = "2023-01-07T12:52:07.538Z" },
        ]
    "#})
        .unwrap();

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
        .arg("--script")
        .arg("script.py")
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
async fn audit_script_vulnerability_found() {
    let context = fyn_test::test_context!("3.12");

    let script = context.temp_dir.child("script.py");
    script
        .write_str(indoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = [
        #   "iniconfig==2.0.0",
        # ]
        # ///
        import iniconfig
    "#})
        .unwrap();

    let lockfile = context.temp_dir.child("script.py.lock");
    lockfile
        .write_str(indoc! {r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [manifest]
        requirements = [{ name = "iniconfig", specifier = "==2.0.0" }]

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3", size = 4646, upload-time = "2023-01-07T12:52:09.585Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beebd3ce04b132a8bf3491/iniconfig-2.0.0-py3-none-any.whl", hash = "sha256:b6a85871a79d2e3b22d2d1b94ac2824226a63c6b741c88f7ae975f18b6778374", size = 5892, upload-time = "2023-01-07T12:52:07.538Z" },
        ]
    "#})
        .unwrap();

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
            }],
            "references": [{
                "type": "ADVISORY",
                "url": "https://example.com/advisory/PYSEC-2023-0001"
            }]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--script")
        .arg("script.py")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: false
    exit_code: 1
    ----- stdout -----

    Vulnerabilities:

    iniconfig 2.0.0 has 1 known vulnerability:

    - PYSEC-2023-0001: A test vulnerability in iniconfig

      Fixed in: 2.1.0

      Advisory information: https://example.com/advisory/PYSEC-2023-0001


    ----- stderr -----
    Found 1 known vulnerability and no adverse project statuses in 1 package
    ");
}

#[tokio::test]
async fn audit_script_frozen_missing_lockfile() {
    let context = fyn_test::test_context!("3.12");

    let script = context.temp_dir.child("script.py");
    script
        .write_str(indoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = [
        #   "iniconfig==2.0.0",
        # ]
        # ///
        import iniconfig
    "#})
        .unwrap();

    let server = MockServer::start().await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--script")
        .arg("script.py")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Unable to find lockfile at `script.py.lock`, but `--frozen` was provided. To create a lockfile, run `fyn lock` or `fyn sync` without the flag.
    ");
}

#[tokio::test]
async fn audit_script_extras() {
    let context = fyn_test::test_context!("3.12");

    let script = context.temp_dir.child("script.py");
    script
        .write_str(indoc! {r#"
        # /// script
        # requires-python = ">=3.12"
        # dependencies = [
        #   "iniconfig[test]",
        # ]
        # ///
        import iniconfig
    "#})
        .unwrap();

    let lockfile = context.temp_dir.child("script.py.lock");
    lockfile
        .write_str(indoc! {r#"
        version = 1
        revision = 3
        requires-python = ">=3.12"

        [manifest]
        requirements = [{ name = "iniconfig", extras = ["test"] }]

        [[package]]
        name = "iniconfig"
        version = "2.0.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/d7/4b/cbd8e699e64a6f16ca3a8220661b5f83792b3017d0f79807cb8708d33913/iniconfig-2.0.0.tar.gz", hash = "sha256:2d91e135bf72d31a410b17c16da610a82cb55f6b0477d1a902134b24a455b8b3", size = 4646, upload-time = "2023-01-07T12:52:09.585Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/ef/a6/62565a6e1cf69e10f5727360368e451d4b7f58beebd3ce04b132a8bf3491/iniconfig-2.0.0-py3-none-any.whl", hash = "sha256:b6a85871a79d2e3b22d2d1b94ac2824226a63c6b741c88f7ae975f18b6778374", size = 5892, upload-time = "2023-01-07T12:52:07.538Z" },
        ]

        [package.optional-dependencies]
        test = [
            { name = "typing-extensions" },
        ]

        [[package]]
        name = "typing-extensions"
        version = "4.10.0"
        source = { registry = "https://pypi.org/simple" }
        sdist = { url = "https://files.pythonhosted.org/packages/16/3a/0d26ce356c7465a19c9ea8f4940a02f85f2b05e6e22cf5fd8a7c1d3b5e0b/typing_extensions-4.10.0.tar.gz", hash = "sha256:b0abd7c89e8fb96f98db18d86106ff1d90ab692004eb746cf6eda2682f91b3cb", size = 77558, upload-time = "2024-02-24T00:10:00.000Z" }
        wheels = [
            { url = "https://files.pythonhosted.org/packages/f9/de/dc04a3ea60b22624b51c703a84bbe0184abcd1d0b9bc8074b5d6b7ab90bb/typing_extensions-4.10.0-py3-none-any.whl", hash = "sha256:69b1a937c3a517342112fb4c6df7e72fc39a38e7891a5730ed4985b5214b5475", size = 33926, upload-time = "2024-02-24T00:09:57.000Z" },
        ]
    "#})
        .unwrap();

    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/querybatch"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "results": [{"vulns": []}, {"vulns": []}]
        })))
        .mount(&server)
        .await;

    fyn_snapshot!(context.filters(), context
        .audit()
        .arg("--frozen")
        .arg("--preview")
        .arg("--script")
        .arg("script.py")
        .arg("--service-url")
        .arg(server.uri()), @"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Found no known vulnerabilities and no adverse project statuses in 2 packages
    ");
}
