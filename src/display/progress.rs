use color_eyre::eyre::Result;
// progress.rs
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;

use crate::display::display_job;
use crate::models::types::PerformIcmpResponse;
use crate::options::Opts;

pub struct ProgressDisplay {
    bar: ProgressBar,
    config: &'static Opts,

    rx: Receiver<PerformIcmpResponse>,
}

#[derive(Clone)]
pub struct ProgressUpdater {
    bar: ProgressBar,
    tx: Sender<PerformIcmpResponse>,
}

impl Drop for ProgressUpdater {
    fn drop(&mut self) {
        self.bar.finish_with_message("Completed");
    }
}

impl ProgressUpdater {
    pub(crate) async fn display_job(
        &self,
        job: progenitor::progenitor_client::ResponseValue<
            crate::models::types::PerformIcmpResponse,
        >,
    ) {
        let _ = self.tx.send(job.into_inner()).await;
        self.bar.inc(1);
    }
}

impl ProgressDisplay {
    pub fn new(config: &'static Opts) -> Result<(Self, ProgressUpdater)> {
        let mut spinner_style = ProgressStyle::default_spinner()
            .tick_chars("-\\|/")
            .template("{spinner:.green} {msg:.cyan/blue} [{elapsed_precise}] {pos}/{len}")?;

        let bar = ProgressBar::new((config.attempts * config.regions.len()) as u64);

        let world_ticker = format!("{}", console::Emoji("🌍🌍🌎🌎🌏🌏🌎🌎🌍🌍", "-\\|/"));
        spinner_style = spinner_style.tick_chars(&world_ticker);
        bar.enable_steady_tick(Duration::from_millis(350));

        bar.set_style(spinner_style);

        let (tx, rx) = mpsc::channel(config.concurrency);

        Ok((
            Self {
                bar: bar.clone(),
                config,
                rx,
            },
            ProgressUpdater { bar, tx },
        ))
    }

    pub async fn display_job_thread(&mut self) {
        while let Some(x) = self.rx.recv().await {
            display_job(&self.bar, self.config, x).await;
        }
    }
}
