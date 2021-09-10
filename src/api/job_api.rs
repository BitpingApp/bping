use std::{thread, time::Duration};
use reqwest::{Client, StatusCode};
use crate::models::{CreateJobAPIRequest, CreateJobAPIResponse, GetJobAPIResponse};

use super::JobErrors;

pub async fn get_job_results(job_id: &str, auth_token: &str) -> Result<GetJobAPIResponse, JobErrors> {
    let job_response: GetJobAPIResponse;
    loop {
        
        let response = Client::new()
            .get(&format!("https://api.bitping.com/job/{}", job_id))
            .bearer_auth(auth_token)
            .send()
            .await?;

        if let Err(e) = response.error_for_status_ref() {
            let err_msg = match response.text().await {
                Ok(v) => v,
                Err(_) => {
                    continue;
                    // return Err(e.into())
                }
            };

            log::debug!("{} {}", e, err_msg);
            continue;
            // return Err(JobErrors::CustomHttpError(e, err_msg))
        }

        let parsed_job_response: GetJobAPIResponse = response.json().await?;    
        if parsed_job_response.job_responses.len() > 0 || parsed_job_response.status == "done" {
            job_response = parsed_job_response;
            break;
        }

        thread::sleep(Duration::from_millis(350));
    }

    Ok(job_response)
}

pub async fn post_job(request: &CreateJobAPIRequest, token: &str) -> Result<CreateJobAPIResponse, JobErrors> {
    let res = Client::new()
      .post("https://api.bitping.com/job")
      .bearer_auth(token)
      .json(&request)
      .send()
      .await?;

      match res.status() {
        StatusCode::OK => (),
        StatusCode::PAYMENT_REQUIRED => return Err(JobErrors::LowFunds),
        status => return Err(JobErrors::OtherFailedStatus(status))
      }
  
      let api_response: CreateJobAPIResponse = res.json().await?;
  
      Ok(api_response)
}