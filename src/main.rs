use std::{pin, marker};

use models::GetJobAPIResponse;
use futures::{StreamExt};

use clap::ArgMatches;
use clap::{Arg, App};
use log;
use indicatif::{ProgressBar, ProgressStyle};


mod models;
mod authentication;
mod display;
mod api;
mod app_config;
mod custom_validators;


#[tokio::main]
async fn main() {
    fern::Dispatch::new()
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply().unwrap();

    let matches = App::new("Bping")
                    .version("1.0")
                    .author("Bitping Team")
                    .about("A command line utility to ping a website from anywhere in the world!")
                    .after_help("Extra configuration can be found at ~/.bitping/.bpingrc")
                    .arg(Arg::with_name("endpoint")
                               .value_name("endpoint")
                               .help("Specifies the endpoint (without http://) to ping. eg. bitping.com")
                               .required(true)
                               .index(1)
                               .takes_value(true))
                    .arg(Arg::with_name("regions")
                            .value_name("regions")
                            .long("regions")
                            .short("r")
                            .help("Specifies the ISO 3166 country codes / continent codes to send jobs to. Defaults to Worldwide.")
                            .value_delimiter(",")
                            .validator(|x| -> Result<(), String> {
                                match custom_validators::validate_region(&x) {
                                    Some(_) => Ok(()),
                                    None => Err(format!("Given region is invalid: {}", x))
                                }
                            })
                            .takes_value(true))
                    .arg(Arg::with_name("count")
                            .value_name("count")
                            .long("count")
                            .short("c")
                            .help("Specifies the number jobs to send. Defaults to 1.")
                            .default_value("1")
                            .takes_value(true))
                    .get_matches();

    let default_configuration = app_config::get_configuration();

    let endpoint = matches.value_of("endpoint").unwrap_or("google.com");
    log::debug!("Value for endpoint: {}", endpoint);

    let mut parsed_regions = parse_regions(matches.clone());
    if parsed_regions.len() == 0 {
        parsed_regions = default_configuration.default_regions;
    }
    
    log::debug!("Value for regions: {} {}", parsed_regions.join(","), matches.clone().occurrences_of("regions"));

    let count_str = matches.value_of("count").map_or_else(|| "1", |x| x.trim());
    let count: u64 = count_str.parse::<u64>().map_or_else(|e| {
        log::error!("Error occurred when parsing number: {} ", e);
        1
    }, |x| x);

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

    let progress_bar_string = display::get_progress_bar_text(endpoint, &parsed_regions);

    pb.set_message(&progress_bar_string);

    

    let request = models::CreateJobAPIRequest {
        job_type: String::from("ping"),
        endpoint: String::from(endpoint),
        regions: parsed_regions
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
}

fn parse_regions(matches: ArgMatches) -> std::vec::Vec<String> {
    if let Some(val) = matches.values_of("regions") {
        let collected = val.collect::<Vec<&str>>();
        return collected.into_iter().map(|x| x.to_string()).collect()
    }

    vec!()
}
