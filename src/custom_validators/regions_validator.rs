use celes::Country;
use super::ValidationErrors::{self};

fn get_continent_code(continent: &str) -> Option<String> {
  match continent.to_uppercase().as_ref() {
    "OCEANIA" => Some("OC".to_string()),
    "ASIA" => Some("AS".to_string()),
    "AFRICA" => Some("AF".to_string()),
    "EUROPE" => Some("EU".to_string()),
    "NORTH AMERICA" => Some("NA".to_string()),
    "SOUTH AMERICA" => Some("SA".to_string()),
    "ANTARCTICA" => Some("AN".to_string()),
    _ => None
  }
}

fn get_continent_name(continent_code: &str) -> Option<String> {
  match continent_code.to_uppercase().as_ref() {
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

pub fn is_continent(region: &str) -> bool {
  if let Some(_) = get_continent_code(region) {
    return true
  }

  if let Some(_) = get_continent_name(region) {
    return true
  }

  return false
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
    return Some(country_info.long_name.into())
  }

  // Lookup country code by name
  if let Ok(country_info) = Country::from_name(region) {
    return Some(country_info.code.into())
  }

  None
}

pub fn get_emoji_safe_region_code(region: &str) -> Option<String> {
  if let Some(_) = get_continent_code(region) {
    return Some(region.to_uppercase());
  };

  if let Some(name) = get_continent_name(region) {
    return Some(name.to_uppercase());
  };

  if let Ok(country_info) = Country::from_name(region) {
    return Some(country_info.alpha2.to_uppercase())
  };

  if let Ok(country_info) = Country::from_alpha2(region) {
    return Some(country_info.alpha2.to_uppercase())
  };

  return None;
}