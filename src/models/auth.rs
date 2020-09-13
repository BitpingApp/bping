use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BitpingConfig {
  pub id: String,
  pub name: String,
  pub email: String,
  pub token: String
}