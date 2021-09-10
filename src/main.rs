use colorful::Colorful;
use std::{pin, thread, time::Duration};

use models::GetJobAPIResponse;
use futures::{StreamExt, stream};

use log;
use indicatif::{ProgressBar, ProgressStyle};
use clap::Clap;

use crate::errors::BpingErrors;

mod errors;
mod models;
mod authentication;
mod display;
mod api;
mod app_config;
mod custom_validators;
mod options;

#[tokio::main]
async fn main() -> Result<(), BpingErrors> {
    let default_configuration = app_config::get_configuration();

    fern::Dispatch::new()
        // Add blanket level filter -
        .level(match default_configuration.diagnostics.log_level {
            0 => log::LevelFilter::Debug,
            1 => log::LevelFilter::Trace,
            2 => log::LevelFilter::Info,
            3 => log::LevelFilter::Warn,
            4 => log::LevelFilter::Error,
            _ => log::LevelFilter::Off
        })
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply().unwrap();

    let opts = options::Opts::parse();

    let endpoint = opts.endpoint;
    log::debug!("Value for endpoint: {}", endpoint);

    let mut parsed_regions = opts.regions;
    if parsed_regions.len() == 0 {
        parsed_regions = default_configuration.default_regions;
    }

    let final_regions = parsed_regions.to_owned();
    check_node_availability(parsed_regions).await?;
    
    let count: u64 = opts.count;

    log::debug!("Number of jobs to send is {}", count);
    
    let token = authentication::retrieve_user_token().await?;

    let mut spinner_style = ProgressStyle::default_spinner()
        .tick_chars("-\\|/")
        .template("{spinner:.green} {msg:.cyan/blue} [{elapsed_precise}] {pos}/{len}");
    
    let pb = ProgressBar::new(count);    
    

    if default_configuration.show_emojis == true {
        let world_ticker = format!("{}", console::Emoji("🌍🌎🌏🌏🌎🌍", "-\\|/"));
        spinner_style = spinner_style.tick_chars(&world_ticker);
        pb.enable_steady_tick(350);
    } else {
        pb.enable_steady_tick(100);
    }

    pb.set_style(spinner_style);

    let progress_bar_string = display::get_progress_bar_text(&endpoint, &final_regions);

    pb.set_message(progress_bar_string);

    let request = models::CreateJobAPIRequest {
        job_type: String::from("ping"),
        endpoint: endpoint.to_string(),
        regions: final_regions
    };

    let urls = vec![request; count as usize];
    let fetches = stream::iter(urls.iter());

    // Pin the progress bar pointer to ensure it doesnt move in memory when we are using it asynchronously
    // let pb_ptr = pin::Pin::new(&pb);
    fetches.for_each_concurrent(10, |api_req| {
        let inner_token = &token;
        let pb = &pb;

        async move {
            let resp = match api::post_job(&api_req, &inner_token).await {
                Ok(v) => v,
                Err(e) => {
                    pb.inc(1);
                    return log::error!("{}", e)
                }
            };

            tokio::time::sleep(Duration::from_millis(1000)).await;

            let job_id  = resp.id;
            let api_res = match api::get_job_results(&job_id, &inner_token).await {
                Ok(v) => v,
                Err(e) => {
                    pb.inc(1);
                    return log::error!("{}", e)
                }
            };

            display::display_job(pb, &api_res, &inner_token).await;
            
            pb.inc(1);
        }
    }).await;

    pb.finish();
    Ok(())
}

async fn perform_job(req: &models::CreateJobAPIRequest, token: &str) -> Result<GetJobAPIResponse, BpingErrors> {
    let resp = api::post_job(req, token).await?;
    let job_id  = resp.id;
    let res = api::get_job_results(&job_id, token).await?;
    Ok(res)
}

async fn check_node_availability(parsed_regions: Vec<String>) -> Result<(), BpingErrors> {
    if let Ok(available_nodes) = api::get_available_nodes().await {
        for region in parsed_regions {
            if custom_validators::is_continent(&region) {
                continue;
            }

            if let Some(country_code) = custom_validators::get_emoji_safe_region_code(&region) {
                if let None = available_nodes.results.iter().find(|x| x.countrycode == country_code) {
                    return Err(BpingErrors::JobErrors(crate::api::JobErrors::UnableToFindNodes))
                }
            }
        }
    }

    Ok(())
}