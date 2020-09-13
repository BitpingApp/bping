use celes::Country;
use super::ValidationErrors::{self};

fn get_continent_code(continent: &str) -> Option<String> {
  match continent {
    "Oceania" => Some("OC".to_string()),
    "Asia" => Some("AS".to_string()),
    "Africa" => Some("AF".to_string()),
    "Europe" => Some("EU".to_string()),
    "North America" => Some("NA".to_string()),
    "South America" => Some("SA".to_string()),
    "Antarctica" => Some("AN".to_string()),
    _ => None
  }
}

fn get_continent_name(continent_code: &str) -> Option<String> {
  match continent_code {
    "OC" => Some("Oceania".to_string()),
    "AS" => Some("Asia".to_string()),
    "AF" => Some("Africa".to_string()),
    "EU" => Some("Europe".to_string()),
    "NA" => Some("North America".to_string()),
    "SA" => Some("South America".to_string()),
    "AN" => Some("Antarctica".to_string()),
    _ => None
  }
}

pub fn validate_region(region: &str) -> Option<String> {
  if let Some(code) = get_continent_code(region) {
    return Some(code)
  }

  if let Some(name) = get_continent_name(region) {
    return Some(name)
  }

  // Look up country name by code
  if let Ok(country_info) = Country::from_alpha2(region) {
    return Some(country_info.long_name)
  }

  // Lookup country code by name
  if let Ok(country_info) = Country::from_name(region) {
    return Some(country_info.code)
  }

  None
}