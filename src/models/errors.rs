use progenitor::progenitor_client;
use reqwest::StatusCode;
use thiserror::Error;

use super::types::{error::ConversionError, PerformIcmpResponse};

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Unable to call Bitping API. Please set BITPING_API_KEY environment variable.")]
    UnauthorizedError,
    #[error("Some configuration was passed to the request that is not allowed {0}")]
    ImpossibleRequestConfiguration(#[from] ConversionError),
    #[error(transparent)]
    ProgenitorError(progenitor_client::Error<PerformIcmpResponse>),
}

impl From<progenitor_client::Error<PerformIcmpResponse>> for Errors {
    fn from(value: progenitor_client::Error<PerformIcmpResponse>) -> Self {
        match value {
            progenitor_client::Error::UnexpectedResponse(response)
                if response.status() == StatusCode::UNAUTHORIZED =>
            {
                Errors::UnauthorizedError
            }
            e => Errors::ProgenitorError(e),
        }
    }
}
