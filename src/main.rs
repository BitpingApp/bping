use std::sync::LazyLock;

use color_eyre::eyre::{self, Context, ContextCompat};
use display::ProgressDisplay;
use job::{IcmpJob, JobScheduler};
use options::Opts;
use tokio::join;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod display;
mod job;
mod models;
mod options;

static APP_CONFIG: LazyLock<Opts> = LazyLock::new(|| Opts::parser().run());

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let filter = EnvFilter::from_default_env();

    let fmt = tracing_subscriber::fmt::Layer::default()
        .compact()
        .pretty()
        .with_thread_ids(true)
        .with_target(false);
    tracing_subscriber::registry().with(fmt).with(filter).init();

    let (mut progress, updater) = ProgressDisplay::new(&APP_CONFIG)?;
    let scheduler = JobScheduler::new(&APP_CONFIG)?;

    info!(config = ?&APP_CONFIG, "Starting job execution with config");

    let display_driver = progress.display_job_thread();
    let schedule_driver = scheduler.execute_jobs(updater);
    let _ = join!(schedule_driver, display_driver);

    Ok(())
}
