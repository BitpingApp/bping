use clap::Clap;

use crate::custom_validators;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap, Debug)]
#[clap(
  version = "1.0", 
  author = "Bitping Team", 
  name = "Bping", 
  about = "A command line utility to ping a website from anywhere in the world!",
  after_help = "Extra configuration can be found at ~/.bitping/.bpingrc"
)]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(
      name = "endpoint", 
      value_name = "endpoint", 
      index = 1, 
      required = true, 
      takes_value = true,
      about = "Specifies the endpoint (without http://) to ping. eg. bitping.com"
    )]
    pub endpoint: String,

    #[clap(
      name = "regions", 
      value_name = "regions",
      short = "r",
      long = "regions",
      value_delimiter = ",",
      takes_value = true,
      validator = validate_regions,
      about = "Specifies the ISO 3166 country codes / continent codes to send jobs to. Defaults to Worldwide."
    )]
    pub regions: Vec<String>,

    #[clap(
      name = "count", 
      value_name = "count",
      short = "c",
      long = "count",
      required = true, 
      takes_value = true,
      default_value = "1",
      about = "Specifies the number jobs to send. Defaults to 1."
    )]
    pub count: u64,

}

fn validate_regions(x: &str) -> Result<(), String> {
  match custom_validators::validate_region(&x) {
      Some(_) => Ok(()),
      None => Err(format!("Given region is invalid: {}", x))
  }
}