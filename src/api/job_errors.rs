use thiserror::Error;
use reqwest::StatusCode;
use std::io;

#[derive(Error, Debug)]
pub enum JobErrors {
    #[error("Funds are too low to create a job, top up your account at https://app.bitping.com")]
    LowFunds,
    #[error("Failed to create job with error code {1} {0}")]
    OtherFailedStatus(reqwest::Error, StatusCode),
    #[error("Failed to create job request {0}")]
    InvalidRequest(#[from] serde_json::Error),
    #[error("Failed to parse job response {0}")]
    FailedToParseResponse(reqwest::Error),
    #[error(transparent)]
    Other(#[from] reqwest::Error)
}