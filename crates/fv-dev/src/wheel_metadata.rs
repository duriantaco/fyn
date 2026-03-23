use std::str::FromStr;

use anstream::println;
use anyhow::{Result, bail};
use clap::Parser;

use fv_cache::{Cache, CacheArgs};
use fv_client::{BaseClientBuilder, RegistryClientBuilder};
use fv_distribution_filename::WheelFilename;
use fv_distribution_types::{BuiltDist, DirectUrlBuiltDist, IndexCapabilities, RemoteSource};
use fv_pep508::VerbatimUrl;
use fv_pypi_types::ParsedUrl;
use fv_settings::EnvironmentOptions;

#[derive(Parser)]
pub(crate) struct WheelMetadataArgs {
    url: VerbatimUrl,
    #[command(flatten)]
    cache_args: CacheArgs,
}

pub(crate) async fn wheel_metadata(
    args: WheelMetadataArgs,
    environment: EnvironmentOptions,
) -> Result<()> {
    let cache = Cache::try_from(args.cache_args)?.init().await?;
    let client = RegistryClientBuilder::new(
        BaseClientBuilder::default()
            .read_timeout(environment.http_read_timeout)
            .connect_timeout(environment.http_connect_timeout),
        cache,
    )
    .build();
    let capabilities = IndexCapabilities::default();

    let filename = WheelFilename::from_str(&args.url.filename()?)?;

    let ParsedUrl::Archive(archive) = ParsedUrl::try_from(args.url.to_url())? else {
        bail!("Only HTTPS is supported");
    };

    let metadata = client
        .wheel_metadata(
            &BuiltDist::DirectUrl(DirectUrlBuiltDist {
                filename,
                location: Box::new(archive.url),
                url: args.url,
            }),
            &capabilities,
        )
        .await?;
    println!("{metadata:?}");
    Ok(())
}
