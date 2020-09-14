use reqwest::{Client, StatusCode};
use crate::models;

pub async fn get_available_nodes() -> std::result::Result<models::AvailableNodes, anyhow::Error> {
  let response = match Client::new()
          .get("https://api.bitping.com/nodes/countries")
          .send()
          .await {
            Ok(e) => e,
            Err(e) => return Err(anyhow::format_err!("Error getting available nodes {}", e))
          };

  match response.json::<models::AvailableNodes>().await {
    Ok(v) => return Ok(v),
    Err(e) => return Err(anyhow::format_err!("Error parsing available nodes {}", e))
  };
}