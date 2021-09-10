use reqwest::{Client, StatusCode};
use crate::models::{self, AvailableNodes};
use crate::api::JobErrors;

pub async fn get_available_nodes() -> Result<AvailableNodes, JobErrors> {
  let response = Client::new().get("https://api.bitping.com/nodes/countries")
          .send()
          .await?;

  if response.status() != StatusCode::OK {
    return Err(JobErrors::UnableToFindNodes)
  };

  Ok(response.json().await?)
}