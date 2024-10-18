use colorful::{Color, Colorful};

use crate::{
    models::types::{
        PerformIcmpResponseNodeInfo, PerformIcmpResponseResultsItem,
        PerformIcmpResponseResultsItemResult,
    },
    options::Opts,
};
use std::*;

/*
 * PING bitping.com (76.76.21.21): 56 data bytes
64 bytes from 76.76.21.21: icmp_seq=0 ttl=120 time=16.181 ms
64 bytes from 76.76.21.21: icmp_seq=1 ttl=120 time=10.129 ms
64 bytes from 76.76.21.21: icmp_seq=2 ttl=120 time=15.644 ms
^C
--- bitping.com ping statistics ---
4 packets transmitted, 4 packets received, 0.0% packet loss
round-trip min/avg/max/stddev = 10.127/13.020/16.181/2.898 ms
 *
 */
pub fn display_success_ping(
    pb: &indicatif::ProgressBar,
    config: &Opts,
    endpoint: &str,
    jobres: &PerformIcmpResponseResultsItemResult,
    node_info: &PerformIcmpResponseNodeInfo,
) {
    format_ping_header(pb, config, endpoint, &jobres.ip_address, node_info);

    // Display individual ping results
    let trips = jobres.trips as usize;
    for i in 0..trips {
        let time = jobres.min + (jobres.max - jobres.min) * (i as f64 / (trips - 1) as f64);
        pb.println(format!(
            "64 bytes from {}: icmp_seq={} ttl=120 time={:.2} ms",
            jobres.ip_address, i, time
        ));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    pb.println(""); // Empty line for spacing

    // Construct and print statistics line
    pb.println(format!("--- {endpoint} ping statistics ---"));

    // Print packet loss information
    pb.println(format!(
        "{} packets transmitted, {} packets received, {:.1}% packet loss",
        jobres.packets_sent,
        jobres.packets_recv,
        jobres.packet_loss * 100.0
    ));

    // Print round-trip statistics
    pb.println(format!(
        "round-trip min/avg/max/stddev = {:.3}/{:.3}/{:.3}/{:.3} ms",
        jobres.min, jobres.avg, jobres.max, jobres.std_dev
    ));
}

/*
 * PING asdasdasd.com (199.59.242.153): 56 data bytes
Request timeout for icmp_seq 0
Request timeout for icmp_seq 1
Request timeout for icmp_seq 2
Request timeout for icmp_seq 3
^C
--- asdasdasd.com ping statistics ---
5 packets transmitted, 0 packets received, 100.0% packet loss
 */
pub fn display_failed_ping(
    pb: &indicatif::ProgressBar,
    config: &Opts,
    jobres: &PerformIcmpResponseResultsItem,
    node_info: &PerformIcmpResponseNodeInfo,
) {
    let ip_address = jobres
        .result
        .as_ref()
        .map_or("Unknown".to_string(), |r| r.ip_address.clone());
    format_ping_header(pb, config, &jobres.endpoint, &ip_address, node_info);

    // Request timeout for icmp_seq 0, 1, 2, 3
    let attempts = jobres.result.as_ref().map_or(4, |r| r.attempts as usize);
    for index in 0..attempts {
        pb.println(format!("Request timeout for icmp_seq {}", index));
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    // --- asdasdasd.com ping statistics ---
    pb.println(format!("--- {} ping statistics ---", jobres.endpoint));

    // 5 packets transmitted, 0 packets received, 100.0% packet loss
    let error_string = if let Some(result) = &jobres.result {
        format!(
            "{} packets transmitted, {} packets received, {:.1}% packet loss",
            result.packets_sent,
            result.packets_recv,
            result.packet_loss * 100.0
        )
    } else {
        format!(
            "{} packets transmitted, 0 packets received, 100% packet loss",
            attempts
        )
    };
    pb.println(format!("{}", error_string.color(Color::Red)));
}

pub fn format_ping_header(
    pb: &indicatif::ProgressBar,
    config: &Opts,
    endpoint: &str,
    ip_address: &str,
    node_info: &PerformIcmpResponseNodeInfo,
) {
    // PING line
    let ping_line = format!("PING {} ({}): 56 data bytes", endpoint, ip_address);
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
        "├── Origin: {} {}, {} {}",
        country_emoji.unwrap_or(""),
        node_info.region_name.as_deref().unwrap_or("Unknown"),
        country_name,
        coordinates
    );
    pb.println(origin_line);

    // ISP line
    let isp_line = format!("├── ISP: {}", node_info.isp.as_deref().unwrap_or("Unknown"));
    pb.println(isp_line);

    // System line
    let system_line = format!(
        "└── System: {}",
        node_info.operating_system.as_deref().unwrap_or("Unknown")
    );
    pb.println(system_line);

    // Separator line
    pb.println("---");
}
