use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JobErrors {
    #[error("Funds are too low to create a job, top up your account at https://app.bitping.com")]
    LowFunds,
    #[error("Failed to create job with error code {0}")]
    OtherFailedStatus(StatusCode),

    #[error("Unable to find nodes in the specified region")]
    UnableToFindNodes,

    #[error("{0} {1}")]
    CustomHttpError(reqwest::Error, String),

    #[error("{0}")]
    HttpError(#[from] #[source] reqwest::Error),

    #[error("{0}")]
    JsonError(#[from] #[source] std::io::Error),
}