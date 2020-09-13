use std::thread::sleep;
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;

use crate::models::GetJobAPIResponse;

#[path = "../api/mod.rs"]
mod api;
// #[path = "../models/mod.rs"]
// mod models;

#[path = "./ping_display.rs"]
mod ping_display;

pub async fn display_job(pb: &ProgressBar, job_data: &GetJobAPIResponse, token: &str) {  // Job is failed, keep going until done and we have printed everything
  let mut is_done = false;
  let mut index = 0;
  
  let mut results = job_data.clone();
  while !is_done {
      let parsed_time = match results.job_responses[index].response_time.parse::<f32>() {
          Ok(v) => v,
          Err(e) => {
              pb.println(format!("Failed to parse response time {}", e));
              -1.0
          }
      };

      if parsed_time <= -1.0 {
          ping_display::display_failed_ping(pb, &results)
      }
      else {
          ping_display::display_success_ping(pb, &results);
      }

      while results.status != "done" && results.job_responses.len() <= index + 1 {
          let new_job_id = &results.id;
          results = match api::get_job_results(new_job_id, token).await {
              Ok(val) => val,
              Err(e) => {
                  pb.println(format!("Error getting job results {}, trying again", e));
                  return
              }
          }.clone();
          sleep(std::time::Duration::from_millis(350));
      }

      index = index +1;

      if results.status == "done" {
          is_done = true
      }
  }
}