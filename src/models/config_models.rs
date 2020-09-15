use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BpingConfiguration {
  #[serde(default)]
  pub default_regions: Vec<String>,
  #[serde(default)]
  pub show_emojis: bool,
  #[serde(default)]
  pub diagnostics: BpingDiagnosticsConfig
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BpingDiagnosticsConfig {
  #[serde(default)]
  pub show_ping_type: bool,
  #[serde(default)]
  pub log_level: i8
}

impl BpingDiagnosticsConfig {
    fn new(show_ping_type: bool, log_level: i8) -> Self { Self { show_ping_type, log_level } }
}

impl Default for BpingDiagnosticsConfig {
  fn default() -> Self { 
    let diag_config = BpingDiagnosticsConfig::new(false, 3);
    return diag_config;
  }
}