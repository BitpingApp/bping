use std::fmt;
use thiserror::*;

#[derive(Debug, Error)]
pub enum AuthenticationErrors {
    #[error("Unable to log in")]
    LoginError,
    #[error("Provided token has expired, please log in again.")]
    TokenExpired,
    #[error("Provided email is invalid. Please try again.")]
    InvalidEmail,

    #[error("{0}")]
    HttpError(#[from] #[source] reqwest::Error),

    #[error("{0}")]
    CredentialsFile(String),

    #[error("{0}")]
    JsonError(#[from] #[source] std::io::Error),
}