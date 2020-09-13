use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationErrors {
  #[error("Invalid region: {0}, index: {1}")]
  InvalidRegion(String, usize)
}