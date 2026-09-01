//! Types and interfaces for interacting with [OSV] as a vulnerability service.
//!
//! We use OSV's `/v1/querybatch` endpoint to collect vulnerability IDs for all
//! dependencies in a single round-trip (handling pagination as needed), then
//! fetch full vulnerability records from `/v1/vulns/{id}` concurrently.
//!
//! [OSV]: https://osv.dev/

use std::str::FromStr as _;
use std::sync::LazyLock;

use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHashSet};
use tracing::trace;

use crate::types::{self, VulnerabilityID};
use futures::{StreamExt as _, TryStreamExt as _};
use fyn_cache::{Cache, CacheBucket, CacheEntry};
use fyn_client::{CacheControl, CachedClient, CachedClientError};
use fyn_configuration::Concurrency;
use fyn_normalize::PackageName;
use fyn_pep440::Version;
use fyn_redacted::{DisplaySafeUrl, DisplaySafeUrlError};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub static API_BASE: LazyLock<DisplaySafeUrl> = LazyLock::new(|| {
    DisplaySafeUrl::parse("https://api.osv.dev/").expect("embedded OSV URL is a valid URL")
});

/// Errors during OSV service interactions.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error from the cached HTTP client.
    #[error(transparent)]
    Client(#[from] fyn_client::Error),
    /// An error during an HTTP request, including middleware errors.
    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    /// An error when constructing the URL for an API request.
    #[error("Invalid API URL: {0}")]
    Url(DisplaySafeUrl, #[source] DisplaySafeUrlError),
    /// An error when OSV returns an invalid vulnerability record.
    #[error("OSV returned a malformed vulnerability record for `{id}`")]
    MalformedRecord {
        id: String,
        #[source]
        err: reqwest_middleware::Error,
    },
    /// The batch response did not contain exactly one result per query.
    #[error("OSV returned {actual} batch results for {expected} queries")]
    BatchCardinality { expected: usize, actual: usize },
    /// OSV returned a pagination token that was already seen for this dependency.
    #[error("OSV repeated a pagination token for `{package}=={version}`")]
    RepeatedPageToken { package: String, version: String },
    /// OSV returned more pages than the client permits for a single dependency.
    #[error("OSV pagination exceeded the limit of {limit} pages for `{package}=={version}`")]
    PaginationLimit {
        package: String,
        version: String,
        limit: usize,
    },
    /// OSV returned an ID that cannot be represented as one URL path segment.
    #[error("OSV returned an invalid vulnerability ID")]
    InvalidVulnerabilityId,
}

/// Package specification for OSV queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Package {
    /// The package's name.
    name: String,
    /// The package's ecosystem.
    /// For our purposes, this will always be "PyPI".
    ecosystem: String,
}

/// Query request for a single package.
#[derive(Debug, Clone, Serialize)]
struct QueryRequest {
    package: Package,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

/// Event in a vulnerability range.
/// Per the OSV schema, each event object contains exactly one of these event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Event {
    /// A version that introduces the vulnerability.
    Introduced(#[allow(dead_code)] String),
    /// A version that fixes the vulnerability.
    Fixed(String),
    /// The last known affected version.
    LastAffected(#[allow(dead_code)] String),
    /// An upper limit on the range.
    Limit(#[allow(dead_code)] String),
}

/// The type of a version range in an OSV vulnerability record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum RangeType {
    /// The versions in events are SemVer 2.0 versions.
    Semver,
    /// The versions in events are ecosystem-specific.
    /// In our context, this means they're PEP 440 versions.
    Ecosystem,
    /// The versions in events are full-length Git SHAs.
    Git,
    /// Some other range type. We don't expect these in OSV v1 records,
    /// but we include it for forward compatibility.
    /// NOTE: In principle we could use `untagged` here and capture the unknown
    /// type, but there's no value at the moment to doing this (since our processing
    /// of OSV records is limited to just ECOSYSTEM ranges).
    #[serde(other)]
    Other,
}

/// Version range for affected packages.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Range {
    #[serde(rename = "type")]
    range_type: RangeType,
    events: Vec<Event>,
}

/// Package affected by a vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Affected {
    /// Optional for records that identify affected code without package metadata.
    package: Option<Package>,
    ranges: Option<Vec<Range>>,
    // TODO: Enable these fields if/when they contain information that's
    // useful to us, e.g. metadata that constrains a vulnerability to specific
    // Python runtime versions, specific distributions of a version, etc.
    // ecosystem_specific: Option<serde_json::Value>,
    // database_specific: Option<serde_json::Value>,
}

/// The type of a reference in an OSV vulnerability record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum ReferenceType {
    Advisory,
    Article,
    Detection,
    Discussion,
    Report,
    Fix,
    Introduced,
    Package,
    Evidence,
    Web,
    /// Some other reference type. We don't expect these in OSV v1 records,
    /// but we include it for forward compatibility.
    #[serde(other)]
    Other,
}

/// A reference for more information about a vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Reference {
    #[serde(rename = "type")]
    reference_type: ReferenceType,
    url: DisplaySafeUrl,
}

/// A full vulnerability record from OSV.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Vulnerability {
    id: String,
    modified: Timestamp,
    // Note: While the OSV spec says schema_version is required for versions >= 1.0.0,
    // some older records in the database don't have it, so we make it optional.
    // TODO: We could validate that this is 1.x, but the value of doing
    // so is probably limited given that we're strictly checking the shape
    // of the response anyways.
    #[allow(dead_code)]
    schema_version: Option<String>,
    summary: Option<String>,
    details: Option<String>,
    published: Option<Timestamp>,
    affected: Option<Vec<Affected>>,
    aliases: Option<Vec<String>>,
    references: Option<Vec<Reference>>,
}

/// Request body for the batch query API.
#[derive(Debug, Clone, Serialize)]
struct QueryBatchRequest {
    queries: Vec<QueryRequest>,
}

/// A summary of a vulnerability returned by the batch query API.
/// Note: the batch query API only returns IDs and modification timestamps, not full records.
#[derive(Debug, Clone, Deserialize)]
struct VulnSummary {
    id: String,
}

/// One result entry in a batch query response, corresponding to one input query.
#[derive(Debug, Clone, Deserialize)]
struct QueryBatchResult {
    #[serde(default)]
    vulns: Vec<VulnSummary>,
    next_page_token: Option<String>,
}

/// Response from a batch query.
#[derive(Debug, Clone, Deserialize)]
struct QueryBatchResponse {
    results: Vec<QueryBatchResult>,
}

/// Filter for OSV queries.
#[derive(Debug, Copy, Clone)]
pub enum Filter {
    /// Return all vulnerabilities.
    All,
    /// Return only vulnerabilities matching the `MAL-` prefix.
    Malware,
}

impl Filter {
    /// Returns `true` if the given vulnerability ID matches this filter.
    fn matches(self, id: &str) -> bool {
        match self {
            Self::All => true,
            Self::Malware => id.starts_with("MAL-"),
        }
    }
}

/// Synthetic `Cache-Control` header for vulnerability record caching (10 minutes).
///
/// This is injected into responses from OSV (which sends no cache headers)
/// so that the [`CachedClient`] middleware handles caching transparently.
///
/// We use a TTL of 10 minutes for alignment with PyPI.
const VULN_CACHE_CONTROL: &str = "max-age=600";

/// Maximum number of pages accepted for any single dependency query.
///
/// OSV page tokens are opaque and the service does not document a fixed upper bound. This generous
/// limit prevents a faulty or malicious endpoint from keeping an audit in an unbounded request loop.
const MAX_PAGES_PER_QUERY: usize = 100;

/// A dependency query that still has an OSV page to fetch.
struct PendingQuery<'a> {
    dependency_index: usize,
    dependency: &'a types::Dependency,
    page_token: Option<String>,
    pages_fetched: usize,
}

/// Return a deterministic, path-safe cache filename for an untrusted vulnerability ID.
fn vulnerability_cache_filename(base_url: &DisplaySafeUrl, id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(base_url.as_str().as_bytes());
    hasher.update([0]);
    hasher.update(id.as_bytes());
    format!("{:x}.msgpack", hasher.finalize())
}

/// Build an OSV detail URL while treating the vulnerability ID as one opaque path segment.
fn vulnerability_detail_url(base_url: &DisplaySafeUrl, id: &str) -> Result<DisplaySafeUrl, Error> {
    if id.is_empty() || matches!(id, "." | "..") {
        return Err(Error::InvalidVulnerabilityId);
    }
    let mut url = base_url
        .join("v1/vulns/")
        .map_err(|err| Error::Url(base_url.clone(), err))?;
    url.path_segments_mut()
        .expect("a URL that accepts a relative path must have path segments")
        .pop_if_empty()
        .push(id);
    Ok(url)
}

/// Build the public advisory URL while treating the vulnerability ID as one opaque path segment.
fn vulnerability_advisory_url(id: &str) -> DisplaySafeUrl {
    let mut url =
        DisplaySafeUrl::parse("https://osv.dev/vulnerability/").expect("embedded OSV URL is valid");
    url.path_segments_mut()
        .expect("embedded OSV URL has path segments")
        .pop_if_empty()
        .push(id);
    url
}

/// Validate and record a pagination token before scheduling another page.
fn validate_next_page_token(
    dependency: &types::Dependency,
    pages_fetched: usize,
    token: &str,
    seen_tokens: &mut FxHashSet<String>,
) -> Result<(), Error> {
    if pages_fetched >= MAX_PAGES_PER_QUERY {
        return Err(Error::PaginationLimit {
            package: dependency.name().to_string(),
            version: dependency.version().to_string(),
            limit: MAX_PAGES_PER_QUERY,
        });
    }
    if !seen_tokens.insert(token.to_string()) {
        return Err(Error::RepeatedPageToken {
            package: dependency.name().to_string(),
            version: dependency.version().to_string(),
        });
    }
    Ok(())
}

/// Represents [OSV](https://osv.dev/), an open-source vulnerability database.
pub struct Osv {
    base_url: DisplaySafeUrl,
    client: CachedClient,
    concurrency: Concurrency,
    cache: Cache,
}

impl Osv {
    /// Create a new OSV client with the given cached HTTP client and optional base URL.
    ///
    /// If no base URL is provided, the client will default to the official OSV API endpoint.
    /// Positive batch query results are cached to disk. Individual vulnerability records
    /// are cached transparently by the [`CachedClient`].
    pub fn new(
        client: CachedClient,
        base_url: Option<DisplaySafeUrl>,
        concurrency: Concurrency,
        cache: Cache,
    ) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| API_BASE.clone()),
            client,
            concurrency,
            cache,
        }
    }

    /// Return a [`CacheEntry`] for a full vulnerability record.
    fn vuln_cache_entry(&self, id: &str) -> CacheEntry {
        let bucket = self.cache.bucket(CacheBucket::Osv);
        CacheEntry::new(
            bucket.join("vulnerability"),
            vulnerability_cache_filename(&self.base_url, id),
        )
    }

    /// Query OSV for vulnerabilities affecting the given dependencies, returning only vulnerability IDs.
    ///
    /// Returns a mapping from each input dependency to the set of vulnerability IDs affecting it.
    pub async fn query_identifiers<'a>(
        &self,
        dependencies: &'a [types::Dependency],
        filter: Filter,
    ) -> Result<IndexMap<&'a types::Dependency, FxHashSet<VulnerabilityID>>, Error> {
        if dependencies.is_empty() {
            return Ok(IndexMap::default());
        }

        let mut result_map: IndexMap<&types::Dependency, FxHashSet<VulnerabilityID>> =
            IndexMap::default();

        // Initially, each dependency has one pending query with no page token.
        let mut pending: Vec<PendingQuery<'_>> = dependencies
            .iter()
            .enumerate()
            .map(|(dependency_index, dependency)| PendingQuery {
                dependency_index,
                dependency,
                page_token: None,
                pages_fetched: 1,
            })
            .collect();
        let mut seen_page_tokens = vec![FxHashSet::default(); dependencies.len()];

        loop {
            let request = QueryBatchRequest {
                queries: pending
                    .iter()
                    .map(|pending| QueryRequest {
                        package: Package {
                            name: pending.dependency.name().to_string(),
                            ecosystem: "PyPI".to_string(),
                        },
                        version: pending.dependency.version().to_string(),
                        page_token: pending.page_token.clone(),
                    })
                    .collect(),
            };

            let url = self
                .base_url
                .join("v1/querybatch")
                .map_err(|e| Error::Url(self.base_url.clone(), e))?;

            // NOTE: we need `uncached` here to access the underlying
            // client for our POST request.
            let batch_response: QueryBatchResponse = self
                .client
                .uncached()
                .for_host(&url)
                .raw_client()
                .post(url.as_ref())
                .json(&request)
                .send()
                .await?
                .error_for_status()
                .map_err(reqwest_middleware::Error::Reqwest)?
                .json()
                .await
                .map_err(reqwest_middleware::Error::Reqwest)?;

            if batch_response.results.len() != pending.len() {
                return Err(Error::BatchCardinality {
                    expected: pending.len(),
                    actual: batch_response.results.len(),
                });
            }

            let mut next_pending = Vec::new();
            for (pending, batch_result) in pending.into_iter().zip(batch_response.results) {
                let ids = result_map.entry(pending.dependency).or_default();
                ids.extend(
                    batch_result
                        .vulns
                        .into_iter()
                        .filter(|v| filter.matches(&v.id))
                        .map(|v| VulnerabilityID::new(v.id)),
                );
                if let Some(token) = batch_result.next_page_token {
                    validate_next_page_token(
                        pending.dependency,
                        pending.pages_fetched,
                        &token,
                        &mut seen_page_tokens[pending.dependency_index],
                    )?;
                    next_pending.push(PendingQuery {
                        dependency_index: pending.dependency_index,
                        dependency: pending.dependency,
                        page_token: Some(token),
                        pages_fetched: pending.pages_fetched + 1,
                    });
                }
            }

            if next_pending.is_empty() {
                break;
            }
            pending = next_pending;
        }

        Ok(result_map)
    }

    /// Query OSV for vulnerabilities affecting the given dependencies, returning full vulnerability records.
    pub async fn query_batch(
        &self,
        dependencies: &[types::Dependency],
        filter: Filter,
    ) -> Result<Vec<types::Finding>, Error> {
        let dep_vuln_ids = self.query_identifiers(dependencies, filter).await?;

        // Collect unique vuln IDs to minimize fetches.
        let unique_ids: FxHashSet<_> = dep_vuln_ids
            .values()
            .flat_map(|ids| ids.iter())
            .cloned()
            .collect();

        // Fetch full vulnerability records concurrently.
        let vuln_details = futures::stream::iter(unique_ids)
            .map(async |id| {
                let vuln = self.fetch_vuln(id.as_str()).await?;
                Ok::<(VulnerabilityID, Vulnerability), Error>((id, vuln))
            })
            .buffer_unordered(self.concurrency.downloads)
            .try_collect::<FxHashMap<VulnerabilityID, Vulnerability>>()
            .await?;

        // Build findings in dependency order (preserved by IndexMap).
        let findings = dep_vuln_ids
            .iter()
            .flat_map(|(dep, vuln_ids)| {
                vuln_ids.iter().filter_map(|vuln_id| {
                    vuln_details
                        .get(vuln_id)
                        .map(|vuln| Self::vulnerability_to_finding(dep, vuln.clone()))
                })
            })
            .collect();

        Ok(findings)
    }

    /// Fetch a full vulnerability record by ID from OSV.
    ///
    /// Caching is handled transparently by the [`CachedClient`] middleware using
    /// a synthetic `Cache-Control: max-age=600` header, since OSV itself does
    /// not send caching headers.
    async fn fetch_vuln(&self, id: &str) -> Result<Vulnerability, Error> {
        let url = vulnerability_detail_url(&self.base_url, id)?;

        let cache_entry = self.vuln_cache_entry(id);
        let req = self
            .client
            .uncached()
            .for_host(&url)
            .raw_client()
            .get(url.as_ref())
            .build()
            .map_err(reqwest_middleware::Error::Reqwest)?;

        let vuln: Vulnerability = self
            .client
            .get_serde_with_retry(
                req,
                &cache_entry,
                CacheControl::Override(VULN_CACHE_CONTROL),
                async |response| response.json::<Vulnerability>().await,
            )
            .await
            .map_err(|err| match err {
                CachedClientError::Client(err) => Error::Client(err),
                CachedClientError::Callback { err, .. } => Error::MalformedRecord {
                    id: id.to_string(),
                    err: reqwest_middleware::Error::Reqwest(err),
                },
            })?;
        if vuln.id.is_empty() || matches!(vuln.id.as_str(), "." | "..") {
            return Err(Error::InvalidVulnerabilityId);
        }
        Ok(vuln)
    }

    /// Convert an OSV Vulnerability record to a Finding.
    fn vulnerability_to_finding(
        dependency: &types::Dependency,
        vuln: Vulnerability,
    ) -> types::Finding {
        // Extract a link for the advisory. We prefer the first
        // `ADVISORY` reference, then the first `WEB` reference, and then
        // finally we synthesize a URL of `https://osv.dev/vulnerability/<id>`
        // where `<id>` is the vulnerability's ID.
        let link = vuln
            .references
            .as_ref()
            .and_then(|references| {
                references
                    .iter()
                    .find(|reference| matches!(reference.reference_type, ReferenceType::Advisory))
                    .or_else(|| {
                        references.iter().find(|reference| {
                            matches!(reference.reference_type, ReferenceType::Web)
                        })
                    })
                    .map(|reference| reference.url.clone())
            })
            .unwrap_or_else(|| vulnerability_advisory_url(&vuln.id));

        // Extract fix versions from affected ranges
        let fix_versions = vuln
            .affected
            .iter()
            .flatten()
            .filter(|affected| {
                // The batch response already associated this vulnerability with the queried
                // dependency, so preserve that association when the detail omits its package.
                affected.package.as_ref().is_none_or(|package| {
                    package.ecosystem.eq_ignore_ascii_case("PyPI")
                        && (package.name == "*"
                            || PackageName::from_str(&package.name)
                                .is_ok_and(|name| &name == dependency.name()))
                })
            })
            .flat_map(|affected| affected.ranges.iter().flatten())
            .filter(|range| matches!(range.range_type, RangeType::Ecosystem))
            .flat_map(|range| &range.events)
            .filter_map(|event| match event {
                // TODO: Warn on a malformed version string rather than silently skipping it.
                // Alternatively, we could propagate the raw version string in the finding and
                // leave it to the callsite to process into PEP 440 versions.
                Event::Fixed(fixed) => {
                    if let Ok(fixed) = Version::from_str(fixed) {
                        Some(fixed)
                    } else {
                        trace!(
                            "Skipping invalid (non-PEP 440) version in OSV record {id}: {fixed}",
                            id = vuln.id,
                        );
                        None
                    }
                }
                _ => None,
            })
            .collect();

        // Extract aliases
        let aliases = vuln
            .aliases
            .unwrap_or_default()
            .into_iter()
            .map(types::VulnerabilityID::new)
            .collect();

        types::Finding::Vulnerability(
            types::Vulnerability::new(
                dependency.clone(),
                types::VulnerabilityID::new(vuln.id),
                vuln.summary,
                vuln.details,
                Some(link),
                fix_versions,
                aliases,
                vuln.published,
                Some(vuln.modified),
            )
            .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use fyn_cache::Cache;
    use fyn_client::{BaseClientBuilder, CachedClient};
    use fyn_configuration::Concurrency;
    use fyn_normalize::PackageName;
    use fyn_pep440::Version;
    use fyn_redacted::DisplaySafeUrl;
    use serde_json::json;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::service::osv::{Error, Filter, RangeType};
    use crate::types::{Dependency, Finding};

    use super::{
        Event, MAX_PAGES_PER_QUERY, Osv, Vulnerability, validate_next_page_token,
        vulnerability_advisory_url, vulnerability_cache_filename, vulnerability_detail_url,
    };

    /// Create a [`CachedClient`] suitable for tests (no retries, no cache).
    fn test_client() -> CachedClient {
        CachedClient::new(
            BaseClientBuilder::default()
                .build()
                .expect("Failed to build test client"),
        )
    }

    #[test]
    fn test_deserialize_events() {
        let json = r#"[{ "introduced": "0" }, { "fixed": "46.0.5" }]"#;
        let events: Vec<Event> = serde_json::from_str(json).expect("Failed to deserialize events");

        insta::assert_debug_snapshot!(events, @r#"
        [
            Introduced(
                "0",
            ),
            Fixed(
                "46.0.5",
            ),
        ]
        "#);
    }

    #[test]
    fn test_deserialize_rangetype() {
        let json = r#"[
          "SEMVER",
          "ECOSYSTEM",
          "GIT",
          "OTHER",
          "UNKNOWN_TYPE"
        ]"#;

        let types: Vec<RangeType> =
            serde_json::from_str(json).expect("Failed to deserialize range types");

        insta::assert_debug_snapshot!(types, @"
        [
            Semver,
            Ecosystem,
            Git,
            Other,
            Other,
        ]
        ");
    }

    #[test]
    fn test_vulnerability_cache_filename_is_deterministic_and_path_safe() {
        let base_url = DisplaySafeUrl::parse("https://example.com/osv/").unwrap();
        let malicious_id = "../../outside/../../../tmp/owned";
        let filename = vulnerability_cache_filename(&base_url, malicious_id);
        let digest = filename
            .strip_suffix(".msgpack")
            .expect("cache filename should have the expected extension");

        assert_eq!(digest.len(), 64);
        assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(
            filename,
            vulnerability_cache_filename(&base_url, malicious_id)
        );
        assert_ne!(
            filename,
            vulnerability_cache_filename(
                &DisplaySafeUrl::parse("https://other.example/osv/").unwrap(),
                malicious_id,
            )
        );

        let osv = Osv::new(
            test_client(),
            Some(base_url),
            Concurrency::default(),
            Cache::temp().unwrap(),
        );
        let entry = osv.vuln_cache_entry(malicious_id);
        let expected_dir = osv
            .cache
            .bucket(fyn_cache::CacheBucket::Osv)
            .join("vulnerability");
        assert_eq!(entry.dir(), expected_dir);
    }

    #[test]
    fn test_vulnerability_urls_treat_ids_as_opaque_path_segments() {
        let id = "../../outside?query=yes#fragment";
        let detail = vulnerability_detail_url(
            &DisplaySafeUrl::parse("https://example.com/osv/").unwrap(),
            id,
        )
        .unwrap();
        assert_eq!(detail.query(), None);
        assert_eq!(detail.fragment(), None);
        assert!(detail.path().starts_with("/osv/v1/vulns/"));
        assert!(
            detail
                .path()
                .ends_with("..%2F..%2Foutside%3Fquery=yes%23fragment")
        );

        let advisory = vulnerability_advisory_url(id);
        assert_eq!(advisory.query(), None);
        assert_eq!(advisory.fragment(), None);
        assert!(
            advisory
                .path()
                .ends_with("..%2F..%2Foutside%3Fquery=yes%23fragment")
        );

        for invalid in ["", ".", ".."] {
            assert!(matches!(
                vulnerability_detail_url(
                    &DisplaySafeUrl::parse("https://example.com/osv/").unwrap(),
                    invalid,
                ),
                Err(Error::InvalidVulnerabilityId)
            ));
        }
    }

    /// Ensure that `query_identifiers` returns the correct vulnerability ID mapping.
    #[tokio::test]
    async fn test_query_identifiers() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                    },
                    {
                        "package": { "name": "package-b", "ecosystem": "PyPI" },
                        "version": "2.0.0",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    { "vulns": [
                        { "id": "VULN-1", "modified": "2026-01-01T00:00:00Z" },
                        { "id": "VULN-3", "modified": "2026-01-03T00:00:00Z" }
                    ] },
                    { "vulns": [
                        { "id": "VULN-2", "modified": "2026-01-02T00:00:00Z" }
                    ] }
                ]
            })))
            .mount(&server)
            .await;

        let osv = Osv::new(
            test_client(),
            Some(DisplaySafeUrl::parse(&server.uri()).unwrap()),
            Concurrency::default(),
            Cache::temp().unwrap(),
        );

        let dependencies = vec![
            Dependency::new(
                PackageName::from_str("package-a").unwrap(),
                Version::from_str("1.0.0").unwrap(),
            ),
            Dependency::new(
                PackageName::from_str("package-b").unwrap(),
                Version::from_str("2.0.0").unwrap(),
            ),
        ];

        let identifiers = osv
            .query_identifiers(&dependencies, Filter::All)
            .await
            .expect("Failed to query identifiers");

        // package-a should have VULN-1 and VULN-3.
        let pkg_a_ids = identifiers.get(&dependencies[0]).unwrap();
        let mut pkg_a_sorted: Vec<_> = pkg_a_ids
            .iter()
            .map(crate::types::VulnerabilityID::as_str)
            .collect();
        pkg_a_sorted.sort_unstable();
        assert_eq!(pkg_a_sorted, ["VULN-1", "VULN-3"]);

        // package-b should have VULN-2.
        let pkg_b_ids = identifiers.get(&dependencies[1]).unwrap();
        let pkg_b_sorted: Vec<_> = pkg_b_ids
            .iter()
            .map(crate::types::VulnerabilityID::as_str)
            .collect();
        assert_eq!(pkg_b_sorted, ["VULN-2"]);

        // Only 1 querybatch request, no vuln detail fetches.
        assert_eq!(
            server.received_requests().await.unwrap().len(),
            1,
            "Expected one querybatch request"
        );
    }

    /// A short batch response must be rejected instead of silently dropping dependencies.
    #[tokio::test]
    async fn test_query_identifiers_rejects_batch_cardinality_mismatch() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "results": [] })))
            .mount(&server)
            .await;

        let osv = Osv::new(
            test_client(),
            Some(DisplaySafeUrl::parse(&server.uri()).unwrap()),
            Concurrency::default(),
            Cache::temp().unwrap(),
        );
        let dependencies = vec![Dependency::new(
            PackageName::from_str("package-a").unwrap(),
            Version::from_str("1.0.0").unwrap(),
        )];

        let error = osv
            .query_identifiers(&dependencies, Filter::All)
            .await
            .expect_err("a short batch response must fail");
        assert!(matches!(
            error,
            Error::BatchCardinality {
                expected: 1,
                actual: 0
            }
        ));
    }

    /// A repeated token must terminate pagination instead of causing an unbounded request loop.
    #[tokio::test]
    async fn test_query_identifiers_rejects_repeated_page_token() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [{ "next_page_token": "repeat" }]
            })))
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                        "page_token": "repeat",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [{ "next_page_token": "repeat" }]
            })))
            .mount(&server)
            .await;

        let osv = Osv::new(
            test_client(),
            Some(DisplaySafeUrl::parse(&server.uri()).unwrap()),
            Concurrency::default(),
            Cache::temp().unwrap(),
        );
        let dependencies = vec![Dependency::new(
            PackageName::from_str("package-a").unwrap(),
            Version::from_str("1.0.0").unwrap(),
        )];

        let error = osv
            .query_identifiers(&dependencies, Filter::All)
            .await
            .expect_err("a repeated page token must fail");
        assert!(matches!(error, Error::RepeatedPageToken { .. }));
        assert_eq!(server.received_requests().await.unwrap().len(), 2);
    }

    #[test]
    fn test_pagination_limit_rejects_another_page() {
        let dependency = Dependency::new(
            PackageName::from_str("package-a").unwrap(),
            Version::from_str("1.0.0").unwrap(),
        );
        let mut seen_tokens = rustc_hash::FxHashSet::default();

        let error =
            validate_next_page_token(&dependency, MAX_PAGES_PER_QUERY, "next", &mut seen_tokens)
                .expect_err("pagination beyond the configured bound must fail");
        assert!(matches!(
            error,
            Error::PaginationLimit {
                limit: MAX_PAGES_PER_QUERY,
                ..
            }
        ));
    }

    /// Ensure that `query_batch` returns the correct findings for a batch of dependencies
    /// with no pagination (simple case).
    #[tokio::test]
    async fn test_query_batch_basic() {
        let server = MockServer::start().await;

        // Querybatch request for both packages.
        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                    },
                    {
                        "package": { "name": "package-b", "ecosystem": "PyPI" },
                        "version": "2.0.0",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    { "vulns": [{ "id": "VULN-1", "modified": "2026-01-01T00:00:00Z" }] },
                    { "vulns": [{ "id": "VULN-2", "modified": "2026-01-02T00:00:00Z" }] }
                ]
            })))
            .mount(&server)
            .await;

        // Individual vuln detail requests.
        Mock::given(method("GET"))
            .and(path("/v1/vulns/VULN-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "VULN-1",
                "modified": "2026-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v1/vulns/VULN-2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "VULN-2",
                "modified": "2026-01-02T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let osv = Osv::new(
            test_client(),
            Some(DisplaySafeUrl::parse(&server.uri()).unwrap()),
            Concurrency::default(),
            Cache::temp().unwrap(),
        );

        let dependencies = vec![
            Dependency::new(
                PackageName::from_str("package-a").unwrap(),
                Version::from_str("1.0.0").unwrap(),
            ),
            Dependency::new(
                PackageName::from_str("package-b").unwrap(),
                Version::from_str("2.0.0").unwrap(),
            ),
        ];

        let findings = osv
            .query_batch(&dependencies, Filter::All)
            .await
            .expect("Failed to query batch");

        insta::assert_debug_snapshot!(findings, @r#"
        [
            Vulnerability(
                Vulnerability {
                    dependency: Dependency {
                        name: PackageName(
                            "package-a",
                        ),
                        version: "1.0.0",
                    },
                    id: VulnerabilityID(
                        "VULN-1",
                    ),
                    summary: None,
                    description: None,
                    link: Some(
                        DisplaySafeUrl {
                            scheme: "https",
                            cannot_be_a_base: false,
                            username: "",
                            password: None,
                            host: Some(
                                Domain(
                                    "osv.dev",
                                ),
                            ),
                            port: None,
                            path: "/vulnerability/VULN-1",
                            query: None,
                            fragment: None,
                        },
                    ),
                    fix_versions: [],
                    aliases: [],
                    published: None,
                    modified: Some(
                        2026-01-01T00:00:00Z,
                    ),
                },
            ),
            Vulnerability(
                Vulnerability {
                    dependency: Dependency {
                        name: PackageName(
                            "package-b",
                        ),
                        version: "2.0.0",
                    },
                    id: VulnerabilityID(
                        "VULN-2",
                    ),
                    summary: None,
                    description: None,
                    link: Some(
                        DisplaySafeUrl {
                            scheme: "https",
                            cannot_be_a_base: false,
                            username: "",
                            password: None,
                            host: Some(
                                Domain(
                                    "osv.dev",
                                ),
                            ),
                            port: None,
                            path: "/vulnerability/VULN-2",
                            query: None,
                            fragment: None,
                        },
                    ),
                    fix_versions: [],
                    aliases: [],
                    published: None,
                    modified: Some(
                        2026-01-02T00:00:00Z,
                    ),
                },
            ),
        ]
        "#);

        // 1 querybatch + 2 vuln detail fetches.
        assert_eq!(
            server.received_requests().await.unwrap().len(),
            3,
            "Expected one querybatch request and two vuln detail requests"
        );
    }

    /// Ensure that `query_batch` correctly handles pagination: only the deps whose results
    /// included a `next_page_token` are re-queried, with their respective tokens.
    #[tokio::test]
    async fn test_query_batch_pagination() {
        let server = MockServer::start().await;

        // First querybatch request: both packages, no page tokens.
        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                    },
                    {
                        "package": { "name": "package-b", "ecosystem": "PyPI" },
                        "version": "2.0.0",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    {
                        "vulns": [{ "id": "VULN-1", "modified": "2026-01-01T00:00:00Z" }],
                        "next_page_token": "tok1"
                    },
                    {
                        "vulns": [{ "id": "VULN-2", "modified": "2026-01-02T00:00:00Z" }]
                    }
                ]
            })))
            .mount(&server)
            .await;

        // Second querybatch request: only package-a with page token.
        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                        "page_token": "tok1",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    { "vulns": [{ "id": "VULN-3", "modified": "2026-01-03T00:00:00Z" }] }
                ]
            })))
            .mount(&server)
            .await;

        // Individual vuln detail requests.
        Mock::given(method("GET"))
            .and(path("/v1/vulns/VULN-1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "VULN-1",
                "modified": "2026-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v1/vulns/VULN-2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "VULN-2",
                "modified": "2026-01-02T00:00:00Z",
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v1/vulns/VULN-3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "VULN-3",
                "modified": "2026-01-03T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let osv = Osv::new(
            test_client(),
            Some(DisplaySafeUrl::parse(&server.uri()).unwrap()),
            Concurrency::default(),
            Cache::temp().unwrap(),
        );

        let dependencies = vec![
            Dependency::new(
                PackageName::from_str("package-a").unwrap(),
                Version::from_str("1.0.0").unwrap(),
            ),
            Dependency::new(
                PackageName::from_str("package-b").unwrap(),
                Version::from_str("2.0.0").unwrap(),
            ),
        ];

        let findings = osv
            .query_batch(&dependencies, Filter::All)
            .await
            .expect("Failed to query batch");

        // package-a has VULN-1 (page 1) and VULN-3 (page 2); package-b has VULN-2.
        assert_eq!(findings.len(), 3);

        let mut ids: Vec<&str> = findings
            .iter()
            .map(|f| match f {
                Finding::Vulnerability(v) => v.id.as_str(),
                Finding::ProjectStatus(_) => unreachable!(),
            })
            .collect();
        ids.sort_unstable();
        assert_eq!(ids, ["VULN-1", "VULN-2", "VULN-3"]);

        // 2 querybatch requests + 3 vuln detail fetches.
        assert_eq!(
            server.received_requests().await.unwrap().len(),
            5,
            "Expected two querybatch requests and three vuln detail requests"
        );
    }

    /// Ensure that `query_batch` with `Filter::Malware` only fetches `MAL-` prefixed
    /// vulnerability IDs.
    #[tokio::test]
    async fn test_query_batch_malware_filter() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/querybatch"))
            .and(body_json(json!({
                "queries": [
                    {
                        "package": { "name": "package-a", "ecosystem": "PyPI" },
                        "version": "1.0.0",
                    }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "results": [
                    {
                        "vulns": [
                            { "id": "MAL-2026-1234", "modified": "2026-01-01T00:00:00Z" },
                            { "id": "GHSA-xxxx-yyyy", "modified": "2026-01-02T00:00:00Z" }
                        ]
                    }
                ]
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v1/vulns/MAL-2026-1234"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "MAL-2026-1234",
                "modified": "2026-01-01T00:00:00Z",
            })))
            .mount(&server)
            .await;

        let osv = Osv::new(
            test_client(),
            Some(DisplaySafeUrl::parse(&server.uri()).unwrap()),
            Concurrency::default(),
            Cache::temp().unwrap(),
        );

        let dependencies = vec![Dependency::new(
            PackageName::from_str("package-a").unwrap(),
            Version::from_str("1.0.0").unwrap(),
        )];

        let findings = osv
            .query_batch(&dependencies, Filter::Malware)
            .await
            .expect("Failed to query batch");

        let [Finding::Vulnerability(vulnerability)] = findings.as_slice() else {
            panic!("Expected exactly one vulnerability finding");
        };

        assert_eq!(vulnerability.id.as_str(), "MAL-2026-1234");
        assert_eq!(
            server.received_requests().await.unwrap().len(),
            2,
            "Expected one querybatch request and one vulnerability detail request"
        );
    }

    #[test]
    fn test_fix_versions_are_scoped_to_queried_pypi_package() {
        let dependency = Dependency::new(
            PackageName::from_str("package-a").unwrap(),
            Version::from_str("1.0.0").unwrap(),
        );
        let vulnerability: Vulnerability = serde_json::from_value(json!({
            "id": "VULN-1",
            "modified": "2026-01-01T00:00:00Z",
            "affected": [
                {
                    "package": { "name": "Package_A", "ecosystem": "PyPI" },
                    "ranges": [{
                        "type": "ECOSYSTEM",
                        "events": [{ "introduced": "0" }, { "fixed": "2.0.0" }]
                    }]
                },
                {
                    "package": { "name": "*", "ecosystem": "PyPI" },
                    "ranges": [{
                        "type": "ECOSYSTEM",
                        "events": [{ "fixed": "3.0.0" }]
                    }]
                },
                {
                    "package": { "name": "package-b", "ecosystem": "PyPI" },
                    "ranges": [{
                        "type": "ECOSYSTEM",
                        "events": [{ "fixed": "99.0.0" }]
                    }]
                },
                {
                    "package": { "name": "package-a", "ecosystem": "npm" },
                    "ranges": [{
                        "type": "ECOSYSTEM",
                        "events": [{ "fixed": "88.0.0" }]
                    }]
                },
                {
                    "package": { "name": "package-a", "ecosystem": "PyPI" },
                    "ranges": [{
                        "type": "SEMVER",
                        "events": [{ "fixed": "77.0.0" }]
                    }]
                },
            ]
        }))
        .expect("valid OSV vulnerability fixture");

        let Finding::Vulnerability(finding) =
            Osv::vulnerability_to_finding(&dependency, vulnerability)
        else {
            unreachable!();
        };
        assert_eq!(
            finding.fix_versions,
            vec![
                Version::from_str("2.0.0").unwrap(),
                Version::from_str("3.0.0").unwrap()
            ]
        );
    }

    #[test]
    fn test_affected_entry_without_package_uses_queried_dependency() {
        let dependency = Dependency::new(
            PackageName::from_str("package-a").unwrap(),
            Version::from_str("1.0.0").unwrap(),
        );
        let vulnerability = serde_json::from_value::<Vulnerability>(json!({
            "id": "VULN-1",
            "modified": "2026-01-01T00:00:00Z",
            "affected": [{
                "ranges": [{
                    "type": "ECOSYSTEM",
                    "events": [{ "fixed": "2.0.0" }]
                }]
            }]
        }))
        .expect("OSV permits affected entries without package metadata");

        let Finding::Vulnerability(finding) =
            Osv::vulnerability_to_finding(&dependency, vulnerability)
        else {
            unreachable!();
        };
        assert_eq!(
            finding.fix_versions,
            vec![Version::from_str("2.0.0").unwrap()]
        );
    }
}
