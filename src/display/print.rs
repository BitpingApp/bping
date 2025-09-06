use std::sync::Arc;

use indicatif::ProgressBar;
use tracing::{error, warn};

use crate::{models::types::PerformIcmpResponse, options::Opts};

use super::ping_display;

pub async fn display_job(pb: &ProgressBar, config: &'static Opts, job_data: PerformIcmpResponse) {
    for result in &job_data.results {
        if let Some(err) = &result.error {
            error!(?err, "Fatal job error.");
            continue;
        }

        if let Some(job_result) = &result.result {
            let Some(ref node_info) = job_data.node_info else {
                warn!("Unable to get node info for job");
                continue;
            };

            if job_result.packet_loss == 1.0 {
                ping_display::display_failed_ping(&pb, &config, result, node_info).await;
                continue;
            }

            ping_display::display_success_ping(
                &pb,
                &config,
                &result.endpoint,
                job_result,
                node_info,
            )
            .await;
        }

        pb.println("");
    }
}
