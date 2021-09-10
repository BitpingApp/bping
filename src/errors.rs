use thiserror::*;
use crate::{api::JobErrors, app_config::ConfigErrors, custom_validators::ValidationErrors, authentication::AuthenticationErrors};

#[derive(Debug, Error)]
pub enum BpingErrors {
  #[error("{0}")]
  JobErrors(#[from] #[source] JobErrors),
  #[error("{0}")]
  ConfigErrors(#[from] #[source] ConfigErrors),
  #[error("{0}")]
  ValidationErrors(#[from] #[source] ValidationErrors),
  #[error("{0}")]
  AuthenticationErrors(#[from] #[source] AuthenticationErrors)
}