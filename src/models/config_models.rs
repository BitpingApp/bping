use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BpingConfiguration {
  #[serde(default)]
  pub default_regions: Vec<String>,
  #[serde(default)]
  pub show_emojis: bool
}