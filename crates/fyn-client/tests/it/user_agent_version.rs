use std::str::FromStr;

use anyhow::Result;
use insta::{assert_snapshot, with_settings};

use fyn_cache::Cache;
use fyn_client::{BaseClientBuilder, RegistryClientBuilder};
use fyn_redacted::DisplaySafeUrl;
use fyn_version::version;
use url::Url;

/// Return the current fyn version as a regex-escaped string for use in snapshot filters.
fn escaped_version() -> String {
    regex::escape(version())
}

use crate::http_util::start_http_user_agent_server;

#[tokio::test]
async fn test_user_agent_has_version() -> Result<()> {
    // Initialize dummy http server
    let (server_task, addr) = start_http_user_agent_server().await?;

    // Initialize fyn-client
    let cache = Cache::temp()?.init().await?;
    let client = RegistryClientBuilder::new(BaseClientBuilder::default(), cache).build();

    // Send request to our dummy server
    let url = DisplaySafeUrl::from_str(&format!("http://{addr}"))?;
    let res = client
        .cached_client()
        .uncached()
        .for_host(&url)
        .get(Url::from(url))
        .send()
        .await?;

    // Check the HTTP status
    assert!(res.status().is_success());

    // Check User Agent
    let body = res.text().await?;

    // Assert fyn version in user agent
    let version = escaped_version();
    let filters = vec![(version.as_str(), "[VERSION]")];
    with_settings!({
        filters => filters
    }, {
        assert_snapshot!(body, @"fyn/[VERSION]");
    });

    // Wait for the server task to complete, to be a good citizen.
    let _ = server_task.await?;

    Ok(())
}
