use dialoguer::*;
use serde_json::json;
use colored::*;

#[path = "../models/mod.rs"]
mod models;
#[path = "./authentication_error.rs"]
mod authentication_error;

pub async fn retrieve_user_token() -> String {
  let home_dir = dirs::home_dir().unwrap();

  // Find bitping config path.
  let bitping_config_path = home_dir.join(".bitping").join("credentials.json");
  if let Ok(bitping_config_str) = std::fs::read_to_string(bitping_config_path) {
    if let Ok(bitping_config) = serde_json::from_str::<models::BitpingConfig>(&bitping_config_str) {
        log::debug!("Bitping token: {}", bitping_config.token);
        return bitping_config.token.to_string();
    }
  }
  
  // Prompt user for username and password
  let mut token: String = String::from("");
  while token == "" {
      log::warn!("{}", "Unable to find your Bitping login credentials locally, please login to continue. \nIf you dont have an account, sign up at https://app.bitping.com/register.".color(Color::Yellow));

      let username = Input::<String>::new()
                              .allow_empty(false)
                              .with_prompt("Please enter your email")
                              .validate_with(|input: &str| -> Result<(), &str> {
                                if validator::validate_email(input) {
                                    Ok(())
                                } else {
                                    Err("This is not a valid email address")
                                }
                            }) 
                              .interact().map_or(String::from(""), |v| v);

      let password = Password::new()
                          .with_prompt("Please enter your password")
                          .allow_empty_password(false)
                          .interact().map_or(String::from(""), |v| v);

      token = match login_with_username_password(&username, &password).await {
          Ok(val) => {
              if let Err(e) = save_credentials_to_disk(&val) {
                log::error!("{}", e);
              };
              val.token
          },
          Err(e) => {
              log::warn!("Login failed, please try again: {}", e);
              String::from("")
          }
      }
  }

  token.to_string()
}

fn save_credentials_to_disk(creds: &models::LoginResponse) -> Result<(), anyhow::Error> {
  let home_dir = dirs::home_dir().unwrap();

  // Find bitping config path.
  let bitping_folder_path = home_dir.join(".bitping");
  let json_str = match serde_json::to_string(creds) {
      Ok(v) => v,
      Err(e) => {
          return Err(anyhow::format_err!("{}", format!("Failed to convert login credentials to json. {}", e).color(Color::Red)))
      }
  };

  let bitping_folder_str = bitping_folder_path.to_str().map_or("", |x| x);
  std::fs::create_dir_all(bitping_folder_path.to_owned())?;

  let credentials_path = bitping_folder_path.join("credentials.json");

  match std::fs::write(credentials_path, &json_str) {
      Ok(_) => {
          log::info!("Successfully wrote credentials to {}", bitping_folder_str);
          Ok(())
      },
      Err(e) => Err(anyhow::format_err!(format!("Failed to write credentials to {} {}", bitping_folder_str, e).color(Color::Red)))
  }
}

pub async fn login_with_username_password(username: &String, password: &String) -> std::result::Result<models::LoginResponse, authentication_error::AuthenticationError> {
  let credentials = json!({
      "email": username.to_string(),
      "password": password.to_string()
  });

  let res = match reqwest::Client::new()
      .post("https://api.bitping.com/users/login")
      .json(&credentials)
      .send()
      .await {
          Ok(val) => val,
          Err(e) => {
              let err_msg = "Error when calling login:\n";
              log::error!("{}{}", err_msg.color(Color::Red), e.to_string().color(Color::Red));
              return Err(authentication_error::AuthenticationError)
          }
      };

    let body_text = match res.text().await {
        Ok(s) => {
            s
        },
        Err(e) => return Err(authentication_error::AuthenticationError)
    };

  let api_response = match serde_json::from_str::<models::LoginResponse>(&body_text) {
      Ok(val) => val,
      Err(e) => {
          let err_msg = "Error when parsing login response:\n";
          log::error!("{}{}", err_msg.color(Color::Red), e.to_string().color(Color::Red));
          return Err(authentication_error::AuthenticationError)
      }
  };

  Ok(api_response)
    // Ok(models::LoginResponse::default())
}
