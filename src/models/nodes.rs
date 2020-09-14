use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AvailableNodes {
  #[serde(rename = "total")]
  pub total: i64,

  #[serde(rename = "results")]
  pub results: Vec<Result>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Result {
  #[serde(rename = "countrycode")]
  pub countrycode: String,

  #[serde(rename = "count")]
  pub count: String,
}
