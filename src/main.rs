use color_eyre::eyre::{self, Context, ContextCompat};
use colorful::Colorful;
use display::display_job;
use keshvar::Alpha2;
use models::{
    types::{
        PerformIcmpBody, PerformIcmpBodyConfiguration, PerformIcmpBodyContinentCode,
        PerformIcmpBodyCountryCode, PerformIcmpBodyMobile, PerformIcmpBodyProxy,
        PerformIcmpBodyResidential, PerformIcmpResponse, PerformIcmpResponseResultsItem,
    },
    Client,
};
use options::Opts;
use progenitor::progenitor_client::ResponseValue;
use rand::seq::SliceRandom;
use reqwest::header::{HeaderMap, HeaderValue};
use std::{
    pin,
    str::FromStr,
    sync::{LazyLock, OnceLock},
    thread,
    time::Duration,
};
use tokio::{sync::mpsc, task::JoinSet};

use futures::{stream, StreamExt};

use indicatif::{ProgressBar, ProgressStyle};
use tracing::{self, debug, info};

mod display;
mod models;
mod options;

static APP_CONFIG: LazyLock<Opts> = LazyLock::new(|| Opts::parser().run());
static API_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-api-key",
        HeaderValue::try_from(&APP_CONFIG.api_key).expect("Unable to parse API Key into header"),
    );

    let req_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build HTTP client");
    Client::new_with_client("https://api.bitping.com/v2", req_client)
});

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    let endpoint = &APP_CONFIG.endpoint;
    tracing::debug!("Value for endpoint: {}", &endpoint);

    tracing::debug!(
        "Number of jobs to send is {}",
        APP_CONFIG.attempts * APP_CONFIG.regions.len()
    );

    let mut spinner_style = ProgressStyle::default_spinner()
        .tick_chars("-\\|/")
        .template("{spinner:.green} {msg:.cyan/blue} [{elapsed_precise}] {pos}/{len}")?;

    let pb = ProgressBar::new((APP_CONFIG.attempts * APP_CONFIG.regions.len()) as u64);

    let world_ticker = format!("{}", console::Emoji("🌍🌍🌎🌎🌏🌏🌎🌎🌍🌍", "-\\|/"));
    spinner_style = spinner_style.tick_chars(&world_ticker);
    pb.enable_steady_tick(Duration::from_millis(350));

    pb.set_style(spinner_style);

    for region in &APP_CONFIG.regions {
        for chunk in (0..APP_CONFIG.attempts)
            .collect::<Vec<_>>()
            .chunks(APP_CONFIG.concurrency)
        {
            let mut chunk_set = JoinSet::new();

            for _ in chunk {
                let pb = pb.clone();
                let region = region.clone();
                let endpoint = endpoint.clone();

                chunk_set.spawn(async move {
                    let (country_code, continent_code) = match region {
                        options::EarthRegion::Country(c) => (
                            Some(PerformIcmpBodyCountryCode::from_str(
                                &c.to_country().alpha2().to_string(),
                            )?),
                            None,
                        ),
                        options::EarthRegion::Continent(con) => (
                            None,
                            Some(PerformIcmpBodyContinentCode::from_str(match con {
                                keshvar::Continent::Africa => "AF",
                                keshvar::Continent::Antarctica => "AN",
                                keshvar::Continent::Asia => "AS",
                                keshvar::Continent::Australia => "OC",
                                keshvar::Continent::Europe => "EU",
                                keshvar::Continent::NorthAmerica => "NA",
                                keshvar::Continent::SouthAmerica => "SA",
                            })?),
                        ),
                        _ => (None, None),
                    };

                    debug!(?country_code, "Sending job to country");

                    let result = API_CLIENT
                        .perform_icmp(&PerformIcmpBody {
                            configuration: Some(PerformIcmpBodyConfiguration {
                                payload_size: Some(56.0),
                                timeout_millis: None,
                                attempts: Some(APP_CONFIG.count as f64),
                            }),
                            country_code,
                            continent_code,
                            hostnames: vec![endpoint.to_string()],
                            isp_regex: None,
                            city: None,
                            mobile: PerformIcmpBodyMobile::from_str("ALLOWED")?,
                            node_id: None,
                            proxy: PerformIcmpBodyProxy::from_str("ALLOWED")?,
                            residential: PerformIcmpBodyResidential::from_str("ALLOWED")?,
                        })
                        .await
                        .context("Failed to send job");

                    pb.inc(1);

                    result
                });
            }

            while let Some(res) = chunk_set.join_next().await {
                match res {
                    Ok(Ok(out)) => {
                        tracing::debug!("Response {:?}", out);
                        display_job(&pb, &APP_CONFIG, &out).await;
                    }
                    Ok(Err(e)) => {
                        tracing::error!("API request failed: {}", e);
                    }
                    Err(e) => {
                        tracing::error!("Task join error: {}", e);
                    }
                }
            }
        }
    }

    pb.finish();
    Ok(())
}
