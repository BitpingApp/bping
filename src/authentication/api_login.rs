use dialoguer::*;
use reqwest::StatusCode;
use serde_json::json;
use colored::*;

use crate::models::{BitpingConfig, LoginResponse};

use super::AuthenticationErrors;
pub async fn retrieve_user_token() -> Result<String, AuthenticationErrors> {
  let home_dir = dirs::home_dir().unwrap();

  // Find bitping config path.
  let bitping_config_path = home_dir.join(".bitping").join("credentials.json");
  if let Ok(bitping_config_str) = std::fs::read_to_string(bitping_config_path) {
    if let Ok(bitping_config) = serde_json::from_str::<BitpingConfig>(&bitping_config_str) {
        log::debug!("Bitping token: {}", bitping_config.token);
        return Ok(bitping_config.token.to_string());
    }
  }
  
  // Prompt user for username and password
  let mut token: String = String::from("");
  while token == "" {
      log::warn!("{}", "Unable to find your Bitping login credentials locally, please login to continue. \nIf you dont have an account, sign up at https://app.bitping.com/register.".color(Color::Yellow));

      let username = Input::<String>::new()
                              .allow_empty(false)
                              .with_prompt("Please enter your email")
                              .validate_with(move |input: &String| -> Result<(), String> {
                                if validator::validate_email(input) {
                                    Ok(())
                                } else {
                                    return Err(AuthenticationErrors::InvalidEmail.to_string())
                                }
                          }) 
                              .interact().map_or(String::from(""), |v| v);

      let password = Password::new()
                          .with_prompt("Please enter your password")
                          .allow_empty_password(false)
                          .interact().map_or(String::from(""), |v| v);

      let creds = login_with_username_password(&username, &password).await?;
      token = creds.token.clone();
      save_credentials_to_disk(&creds)?;
  }

  Ok(token.to_string())
}

fn save_credentials_to_disk(creds: &LoginResponse) -> Result<(), AuthenticationErrors> {
  let home_dir = dirs::home_dir().unwrap();

  // Find bitping config path.
  let bitping_folder_path = home_dir.join(".bitping");
  let json_str = serde_json::to_string(creds).map_err(|e| AuthenticationErrors::CredentialsFile(format!("Failed to convert login credentials to json. {}", e)))?;

  let bitping_folder_str = bitping_folder_path.to_str().map_or("", |x| x);
  std::fs::create_dir_all(bitping_folder_path.to_owned()).map_err(|e| AuthenticationErrors::CredentialsFile(format!("Failed to create bitping directory at {} {}", bitping_folder_str, e)))?;

  let credentials_path = bitping_folder_path.join("credentials.json");

  match std::fs::write(credentials_path, &json_str) {
      Ok(_) => {
          log::info!("Successfully wrote credentials to {}", bitping_folder_str);
          Ok(())
      },
      Err(e) => Err(AuthenticationErrors::CredentialsFile(format!("Failed to write credentials to {} {}", bitping_folder_str, e)))
  }
}

pub async fn login_with_username_password(username: &String, password: &String) -> Result<LoginResponse, AuthenticationErrors> {
  let credentials = &json!({
      "email": username.to_string(),
      "password": password.to_string()
  });

  let res = reqwest::Client::new()
      .post("https://api.bitping.com/users/login")
      .json(&credentials)
      .send()
      .await?;

  match res.status() {
      StatusCode::OK => (),
      status => return Err(AuthenticationErrors::LoginError)
  };

  let api_response = res.json().await?;

  Ok(api_response)
}
