use std::{thread, time::Duration};

use reqwest::{Client, StatusCode};

use crate::models::{CreateJobAPIRequest, CreateJobAPIResponse, GetJobAPIResponse};

#[path = "./job_errors.rs"]
mod job_errors;

pub async fn get_job_results(job_id: &str, auth_token: &str) -> std::result::Result<GetJobAPIResponse, job_errors::JobErrors> {
    let job_response: GetJobAPIResponse;
    loop {
        let response = Client::new()
            .get(&format!("https://api.bitping.com/job/{}", job_id))
            .bearer_auth(auth_token.clone())
            .send()
            .await?;

        let parsed_job_response: GetJobAPIResponse = response.json().await?;    
        if parsed_job_response.job_responses.len() > 0 || parsed_job_response.status == "done" {
            job_response = parsed_job_response;
            break;
        }

        thread::sleep(Duration::from_millis(350));
    }

    Ok(job_response)
}

pub async fn post_job(request: &CreateJobAPIRequest, token: &str) -> std::result::Result<CreateJobAPIResponse, job_errors::JobErrors> {
  let json_obj = serde_json::to_value(request)?;

  let res = match Client::new()
    .post("https://api.bitping.com/job")
    .bearer_auth(token)
    .json(&json_obj)
    .send()
    .await {
        Ok(value) => value,
        Err(e) => {
            if e.is_status() {
                match e.status() {
                    Some(StatusCode::PAYMENT_REQUIRED) => return Err(job_errors::JobErrors::LowFunds),
                    Some(status) => return Err(job_errors::JobErrors::OtherFailedStatus(e, status)),
                    None => {}
                }
            }
            
            return Err(job_errors::JobErrors::Other(e));
        }
    };

    if res.status() == StatusCode::PAYMENT_REQUIRED {
        return Err(job_errors::JobErrors::LowFunds)
    }

    let api_response: CreateJobAPIResponse = res.json::<CreateJobAPIResponse>().await?;

    Ok(api_response)
}