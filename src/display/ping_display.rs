use crate::{
    models::types::{
        PerformIcmpResponseNodeInfo, PerformIcmpResponseResultsItem,
        PerformIcmpResponseResultsItemResult,
    },
    options::Opts,
};
use colorful::{Color, Colorful};
use indicatif::ProgressBar;
use std::*;
use sync::Arc;

fn print_border(pb: &ProgressBar, width: usize) {
    pb.println("┌".to_string() + &"─".repeat(width - 2) + "┐");
}

fn print_footer(pb: &ProgressBar, width: usize) {
    pb.println("└".to_string() + &"─".repeat(width - 2) + "┘");
}

async fn sleep_if_enabled(config: &'static Opts, duration: u64) {
    if !config.no_delay {
        tokio::time::sleep(std::time::Duration::from_millis(duration)).await;
    }
}

pub async fn display_success_ping(
    pb: &ProgressBar,
    config: &'static Opts,
    endpoint: &str,
    jobres: &PerformIcmpResponseResultsItemResult,
    node_info: &PerformIcmpResponseNodeInfo,
) {
    let width = 80;
    print_border(pb, width);
    format_ping_header(pb, config, endpoint, &jobres.ip_address, node_info);

    let trips = jobres.trips as usize;
    for i in 0..trips {
        let time = jobres.min + (jobres.max - jobres.min) * (i as f64 / (trips - 1) as f64);
        pb.println(format!(
            "│ 64 bytes from {}: icmp_seq={} ttl=120 time={:.2} ms",
            jobres.ip_address, i, time
        ));
        sleep_if_enabled(config, time as u64).await;
    }

    pb.println("│");
    pb.println(format!("│ --- {endpoint} ping statistics ---"));

    sleep_if_enabled(config, 250).await;

    pb.println(format!(
        "│ {} packets transmitted, {} packets received, {:.1}% packet loss",
        jobres.packets_sent,
        jobres.packets_recv,
        jobres.packet_loss * 100.0
    ));
    sleep_if_enabled(config, 250).await;

    pb.println(format!(
        "│ round-trip min/avg/max/stddev = {:.3}/{:.3}/{:.3}/{:.3} ms",
        jobres.min, jobres.avg, jobres.max, jobres.std_dev
    ));
    sleep_if_enabled(config, 250).await;

    print_footer(pb, width);
}

pub async fn display_failed_ping(
    pb: &ProgressBar,
    config: &'static Opts,
    jobres: &PerformIcmpResponseResultsItem,
    node_info: &PerformIcmpResponseNodeInfo,
) {
    let width = 80;
    print_border(pb, width);
    let ip_address = jobres
        .result
        .as_ref()
        .map_or("Unknown".to_string(), |r| r.ip_address.clone());
    format_ping_header(pb, config, &jobres.endpoint, &ip_address, node_info);

    let attempts = jobres.result.as_ref().map_or(4, |r| r.attempts as usize);
    for index in 0..attempts {
        pb.println(format!("│ Request timeout for icmp_seq {}", index));
        sleep_if_enabled(config, 500).await;
    }

    pb.println(format!("│ --- {} ping statistics ---", jobres.endpoint));
    sleep_if_enabled(config, 250).await;

    let error_string = if let Some(result) = &jobres.result {
        format!(
            "│ {} packets transmitted, {} packets received, {:.1}% packet loss",
            result.packets_sent,
            result.packets_recv,
            result.packet_loss * 100.0
        )
    } else {
        format!(
            "│ {} packets transmitted, 0 packets received, 100% packet loss",
            attempts
        )
    };
    sleep_if_enabled(config, 250).await;

    pb.println(format!("{}", error_string.color(Color::Red)));
    sleep_if_enabled(config, 250).await;

    print_footer(pb, width);
}

pub fn format_ping_header(
    pb: &ProgressBar,
    config: &Opts,
    endpoint: &str,
    ip_address: &str,
    node_info: &PerformIcmpResponseNodeInfo,
) {
    // PING line
    let ping_line = format!("│ PING {} ({}): 56 data bytes", endpoint, ip_address);
    pb.println(ping_line);

    // Origin line
    let country_emoji = node_info.country_code.as_ref().and_then(|cc| {
        keshvar::Alpha2::try_from(cc.as_str())
            .ok()
            .map(|c| c.to_country().emoji())
    });

    let country_name = keshvar::Alpha2::try_from(node_info.country_code.as_deref().unwrap_or(""))
        .ok()
        .map(|c| c.to_country().iso_short_name())
        .unwrap_or("Unknown");

    let coordinates = match (node_info.lat, node_info.lon) {
        (Some(lat), Some(lon)) => format!("({:.2}°N, {:.2}°E)", lat, lon),
        _ => String::new(),
    };

    let origin_line = format!(
        "│ ├── Origin: {} {}, {} {}",
        country_emoji.unwrap_or(""),
        node_info.region_name.as_deref().unwrap_or("Unknown"),
        country_name,
        coordinates
    );
    pb.println(origin_line);

    // ISP line
    let isp_line = format!(
        "│ ├── ISP: {}",
        node_info.isp.as_deref().unwrap_or("Unknown")
    );
    pb.println(isp_line);

    // System line
    let system_line = format!(
        "│ └── System: {}",
        node_info.operating_system.as_deref().unwrap_or("Unknown")
    );
    pb.println(system_line);

    // Separator line
    pb.println("│ ---");
}
