use crate::{
    display::{ProgressDisplay, ProgressUpdater},
    models::{errors::Errors, *},
    options::{EarthRegion, Opts},
};
use color_eyre::eyre::{self, Context, Result};
use futures::stream::{self, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue};
use std::{iter::repeat, str::FromStr, sync::Arc, time::Duration};
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{debug, error, info};
use types::{
    PerformIcmpBody, PerformIcmpBodyConfiguration, PerformIcmpBodyContinentCode,
    PerformIcmpBodyCountryCode, PerformIcmpBodyMobile, PerformIcmpBodyProxy,
    PerformIcmpBodyResidential, PerformIcmpResponse,
};

#[derive(Debug)]
pub struct IcmpJob {
    config: &'static Opts,
    region: EarthRegion,
}

impl IcmpJob {
    pub fn new(config: &'static Opts, region: EarthRegion) -> Self {
        Self { config, region }
    }

    pub async fn execute(
        &self,
        client: &Client,
    ) -> Result<ResponseValue<PerformIcmpResponse>, Errors> {
        info!(
            region = ?self.region,
            attempts = self.config.count,
            "Executing ICMP job"
        );
        let (country_code, continent_code) = self.region.get_codes()?;

        let request = PerformIcmpBody {
            configuration: Some(PerformIcmpBodyConfiguration {
                payload_size: Some(56.0),
                timeout_millis: None,
                attempts: Some(self.config.count as f64),
            }),
            country_code,
            continent_code,
            hostnames: vec![self.config.endpoint.to_string()],
            isp_regex: None,
            city: None,
            mobile: PerformIcmpBodyMobile::from_str(&self.config.mobile.to_string())?,
            node_id: None,
            proxy: PerformIcmpBodyProxy::from_str(&self.config.proxy.to_string())?,
            residential: PerformIcmpBodyResidential::from_str(
                &self.config.residential.to_string(),
            )?,
        };

        debug!(?request, "Sending ICMP Request");

        self.execute_with_retry(client, request).await
    }

    async fn execute_with_retry(
        &self,
        client: &Client,
        request: PerformIcmpBody,
    ) -> Result<ResponseValue<PerformIcmpResponse>, Errors> {
        let retry_strategy = ExponentialBackoff::from_millis(100)
            .factor(2)
            .max_delay(Duration::from_millis(200))
            .take(3);

        info!("Executing request with retry strategy");
        Retry::spawn(retry_strategy, || async {
            client.perform_icmp(&request).await.map_err(|e| e.into())
        })
        .await
    }
}

pub struct JobScheduler {
    config: &'static Opts,
    client: Client,
}

impl JobScheduler {
    pub fn new(config: &'static Opts) -> Result<Self> {
        info!(
            concurrency = config.concurrency,
            attempts = config.attempts,
            "Initializing job scheduler"
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::try_from(&config.api_key)
                .context("Unable to parse API Key into header")?,
        );

        let req_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build HTTP client")?;
        let client = Client::new_with_client("https://api.bitping.com/v2", req_client);

        Ok(Self { config, client })
    }

    pub async fn execute_jobs(&self, progress: ProgressUpdater) -> Result<(), Error> {
        info!(
            regions = ?self.config.regions,
            "Starting job execution"
        );
        let jobs = self.jobs_iterator();

        let progress = Arc::new(progress);

        stream::iter(jobs)
            .for_each_concurrent(Some(self.config.concurrency), |job| {
                let client = self.client.clone();

                let progress = progress.clone();
                async move {
                    match job.execute(&client).await {
                        Ok(v) => progress.display_job(v).await,
                        Err(Errors::UnauthorizedError) => {
                            error!("{}", Errors::UnauthorizedError);
                        }
                        Err(e) => {
                            error!(?e, "Job failed");
                        }
                    };
                }
            })
            .await;

        Ok(())
    }

    // Replace prepare_jobs with an iterator
    fn jobs_iterator(&self) -> impl Iterator<Item = IcmpJob> + '_ {
        self.config.regions.iter().flat_map(move |region| {
            repeat(region)
                .take(self.config.attempts)
                .map(move |r| IcmpJob::new(self.config, r.clone()))
        })
    }
}
