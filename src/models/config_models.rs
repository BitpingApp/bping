use serde::{Deserialize, Serialize};
use crate::log::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BpingConfiguration {
  #[serde(default)]
  pub default_regions: Vec<String>,
  #[serde(default)]
  pub show_emojis: bool,
  #[serde(default)]
  pub diagnostics: BpingDiagnosticsConfig
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BpingDiagnosticsConfig {
  #[serde(default)]
  pub show_ping_type: bool
}