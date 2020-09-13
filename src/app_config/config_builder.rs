use std::path::PathBuf;
use crate::models;
use super::ConfigErrors;

static mut CONFIG: Option<models::BpingConfiguration> = None;

pub fn get_configuration() -> models::BpingConfiguration {
  unsafe {
    if let Some(config) = CONFIG.clone() {
      return config.clone();
    }
  }
  
  match read_configuration() {
    Ok(config) => {
      let val = config.clone();
      unsafe {
        CONFIG = Some(config)
      };
      return val;
    }
    Err(ConfigErrors::CantCreateFile(e)) => {
      log::error!("CantCreateFile {}", e);
    },
    Err(ConfigErrors::CantParseFile(e)) => {
        log::error!("CantParseFile {}", e);
    }
    Err(ConfigErrors::CantReadFile(e)) => {
        log::error!("CantReadFile {}", e);
    }
    Err(e) => {
        log::error!("An error occurred when fetching config {}", e);
    }
  }

  let default_config = models::BpingConfiguration::default();
  let copy_config = default_config.clone();
  unsafe {
    CONFIG = Some(default_config);
  };
  return copy_config;
}

fn read_configuration() -> anyhow::Result<models::BpingConfiguration, ConfigErrors> {
  let home_dir = match dirs::home_dir() {
    Some(dir) => dir,
    None => return Err(ConfigErrors::Other(anyhow::format_err!("Could not read home directory path")))
  };

  // Find bitping config path.
  let bping_config_path = home_dir.join(".bitping").join(".bpingrc");

  let config_file = match get_config_file(&bping_config_path) {
    Ok(file) => file,
    Err(e) => return Err(e)
  };

  let config = match toml::from_str::<models::BpingConfiguration>(&config_file) {
    Ok(config) => config,
    Err(e) => return Err(ConfigErrors::CantParseFile(e))
  };

  match save_config_file(&config, &bping_config_path) {
    Ok(()) => return Ok(config),
    Err(e) => return Err(e)
  };
}

fn get_config_file(file_path: &PathBuf) -> Result<String, ConfigErrors> {
  let owned_file_path = file_path.clone();
  if let Ok(file_string) = std::fs::read_to_string(&owned_file_path) {
    return Ok(file_string)
  };

  let default_config = models::BpingConfiguration::default();
  let default_settings = match toml::to_string(&default_config) {
    Ok(s) => s,
    Err(e) => return Err(ConfigErrors::Other(anyhow::format_err!("Cannot create default settings file {}", e)))
  };

  let owned_setting_string = default_settings.clone();
  // Failed to read file, create a new file and read that into a string
  match std::fs::write(owned_file_path, default_settings) {
    Ok(()) => {},
    Err(e) => return Err(ConfigErrors::CantCreateFile(e))
  };

  Ok(owned_setting_string)
}

fn save_config_file(config: &models::BpingConfiguration, file_path: &PathBuf) -> Result<(), ConfigErrors> {
  let serialised = match toml::to_string(config) {
    Ok(serialised) => serialised,
    Err(e) => return Err(ConfigErrors::Other(anyhow::format_err!("Could not create default configuration string {}", e)))
  };

  match std::fs::write(file_path, serialised) {
    Ok(()) => return Ok(()),
    Err(e) => return Err(ConfigErrors::CantCreateFile(e))
  };
}