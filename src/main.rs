use colorful::Colorful;
use std::{pin};

use models::GetJobAPIResponse;
use futures::{StreamExt};

use log;
use indicatif::{ProgressBar, ProgressStyle};
use clap::Clap;

mod models;
mod authentication;
mod display;
mod api;
mod app_config;
mod custom_validators;
mod options;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let default_configuration = app_config::get_configuration();

    fern::Dispatch::new()
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
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
    
    let token = authentication::retrieve_user_token().await;

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

    pb.set_message(&progress_bar_string);

    let request = models::CreateJobAPIRequest {
        job_type: String::from("ping"),
        endpoint: endpoint.to_string(),
        regions: final_regions
    };

    let urls = vec![request; count as usize];
    let MAX_CONCURRENCY = 10;

    let fetches = futures::stream::iter(
        urls.into_iter().map(|req| {
            let inner_token = token.to_owned();
            async move {
                let resp = api::post_job(&req, &inner_token).await?;
                let job_id  = resp.id;
                let res = api::get_job_results(&job_id, &inner_token).await?;
                Ok::<GetJobAPIResponse, anyhow::Error>(res)
            }
        }));

    // Pin the progress bar pointer to ensure it doesnt move in memory when we are using it asynchronously
    let pb_ptr = pin::Pin::new(&pb);
    fetches.for_each_concurrent(MAX_CONCURRENCY, |elem|
        async {
            match elem.await {
                Ok(job) => {
                    let thing = pin::Pin::get_ref(pb_ptr);
                    display::display_job(thing, &job, &token).await;
                },
                Err(e) => {
                    log::error!("Something went wrong when placing a job {}", e)
                }
            }
            
            pb.inc(1);
        }
    ).await;

    pb.finish();
    Ok(())
}

async fn perform_job(req: &models::CreateJobAPIRequest, token: &str) -> Result<GetJobAPIResponse, anyhow::Error> {
    let resp = api::post_job(req, token).await?;
    let job_id  = resp.id;
    let res = api::get_job_results(&job_id, token).await?;
    Ok::<GetJobAPIResponse, anyhow::Error>(res)
}

async fn check_node_availability(parsed_regions: Vec<String>) -> Result<(), anyhow::Error> {
    if let Ok(available_nodes) = api::get_available_nodes().await {
        for region in parsed_regions {
            if custom_validators::is_continent(&region) {
                continue;
            }

            if let Some(country_code) = custom_validators::get_emoji_safe_region_code(&region) {
                if let None = available_nodes.results.iter().find(|x| x.countrycode == country_code) {
                    let error_str = format!("No nodes found for given location {}", region);
                    return Err(anyhow::format_err!("{}", error_str.color(colorful::Color::Red)))
                }
            }
        }
    }

    Ok(())
}