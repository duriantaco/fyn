use std::io::{Cursor, Write};
use std::str::FromStr;

use anyhow::Result;
use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};
use zip::CompressionMethod;
use zip::write::SimpleFileOptions;

use fyn_cache::Cache;
use fyn_client::{BaseClientBuilder, RegistryClientBuilder};
use fyn_distribution_filename::WheelFilename;
use fyn_distribution_types::{BuiltDist, DirectUrlBuiltDist, IndexCapabilities};
use fyn_pep508::VerbatimUrl;
use fyn_redacted::DisplaySafeUrl;

fn build_test_wheel() -> Result<Vec<u8>> {
    let cursor = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    zip.start_file("demo-0.1.0.dist-info/METADATA", options)?;
    zip.write_all(b"Metadata-Version: 2.1\nName: demo\nVersion: 0.1.0\n")?;

    zip.start_file("demo-0.1.0.dist-info/WHEEL", options)?;
    zip.write_all(
        b"Wheel-Version: 1.0\nGenerator: fyn-test\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
    )?;

    zip.start_file("demo/__init__.py", options)?;
    zip.write_all(b"__version__ = '0.1.0'\n")?;

    Ok(zip.finish()?.into_inner())
}

#[tokio::test]
async fn remote_metadata_with_and_without_cache() -> Result<()> {
    let cache = Cache::temp()?.init().await?;
    let client = RegistryClientBuilder::new(BaseClientBuilder::default(), cache)
        .build()
        .expect("failed to build registry client");

    // The first run is without cache (the tempdir is empty), the second has the cache from the
    // first run.
    for _ in 0..2 {
        let url = "https://files.pythonhosted.org/packages/00/e5/f12a80907d0884e6dff9c16d0c0114d81b8cd07dc3ae54c5e962cc83037e/tqdm-4.66.1-py3-none-any.whl";
        let filename = WheelFilename::from_str(url.rsplit_once('/').unwrap().1)?;
        let dist = BuiltDist::DirectUrl(DirectUrlBuiltDist {
            filename,
            location: Box::new(DisplaySafeUrl::parse(url)?),
            url: VerbatimUrl::from_str(url)?,
        });
        let capabilities = IndexCapabilities::default();
        let metadata = client.wheel_metadata(&dist, &capabilities).await?;
        assert_eq!(metadata.version.to_string(), "4.66.1");
    }

    Ok(())
}

#[tokio::test]
async fn remote_metadata_falls_back_when_range_response_is_short() -> Result<()> {
    let server = MockServer::start().await;
    let filename = "demo-0.1.0-py3-none-any.whl";
    let wheel = build_test_wheel()?;
    let wheel_len = wheel.len();

    Mock::given(method("HEAD"))
        .and(path(format!("/{filename}")))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Accept-Ranges", "bytes")
                .insert_header("Content-Length", wheel_len.to_string()),
        )
        .mount(&server)
        .await;

    let broken_wheel = wheel.clone();
    Mock::given(method("GET"))
        .and(path(format!("/{filename}")))
        .and(header_exists("Range"))
        .respond_with(move |request: &Request| {
            let range_header = request.headers["Range"].to_str().unwrap();
            let range_spec = range_header.strip_prefix("bytes=").unwrap();
            let (start, end) = range_spec.split_once('-').unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            let advertised = &broken_wheel[start..=end];
            let actual = advertised[..advertised.len() - 1].to_vec();

            ResponseTemplate::new(206)
                .insert_header("Content-Range", format!("bytes {start}-{end}/{wheel_len}"))
                .insert_header("Content-Length", actual.len().to_string())
                .set_body_bytes(actual)
        })
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(format!("/{filename}")))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Length", wheel_len.to_string())
                .set_body_bytes(wheel.clone()),
        )
        .mount(&server)
        .await;

    let cache = Cache::temp()?.init().await?;
    let client = RegistryClientBuilder::new(BaseClientBuilder::default(), cache)
        .build()
        .expect("failed to build registry client");

    let url = format!("{}/{}", server.uri(), filename);
    let dist = BuiltDist::DirectUrl(DirectUrlBuiltDist {
        filename: WheelFilename::from_str(filename)?,
        location: Box::new(DisplaySafeUrl::parse(&url)?),
        url: VerbatimUrl::from_str(&url)?,
    });

    let metadata = client
        .wheel_metadata(&dist, &IndexCapabilities::default())
        .await?;
    assert_eq!(metadata.name.as_ref(), "demo");
    assert_eq!(metadata.version.to_string(), "0.1.0");

    Ok(())
}
