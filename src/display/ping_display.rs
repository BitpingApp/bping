/**
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
use std::*;

use colorful::{Colorful, Color};

use crate::{app_config, display::emojis};


pub fn display_success_ping(pb: &indicatif::ProgressBar, job: &crate::models::GetJobAPIResponse) {
  let config = app_config::get_configuration();

  // if we have ip address show crazy one, otherwise just do the bottom
  // choose random number between min and max to show each line
  for jobres in &job.job_responses {
    // PING bitping.com (76.76.21.21): 56 data bytes
    let mut line_1 = format!("\nPING"); 

    if config.diagnostics.show_ping_type == true {
      if let Some(ping_type) = jobres.data.get("pingType") {
        line_1 = format!("{} ({type})", line_1, type = ping_type);
      }
    }
    
    line_1 = format!("{} {endpoint}", line_1, endpoint = job.endpoint);
    if let Some(ip_address_val) = jobres.data.get("ip_address") {
      let ip_address = ip_address_val.as_str().map_or("", |x| x);
      line_1 = format!("{} ({ip_address})", line_1, ip_address = ip_address);
    }
    line_1 = format!("{}: 56 data bytes", line_1);
    pb.println(line_1);

    // 64 bytes from 76.76.21.21: icmp_seq=0 ttl=120 time=16.181 ms
    if let Some(round_trips) = jobres.data.get("trips") {
      let trips = round_trips.as_array().map_or(vec!(), |x| x.to_vec());
      for (index, rtt) in trips.iter().enumerate() {
        let mut time_line = "64 bytes".to_string();

        if let Some(ip_address_val) = jobres.data.get("ip_address") {
          let ip_address = ip_address_val.as_str().map_or("", |x| x);
          time_line = format!("{} from {ip_address}", time_line, ip_address = ip_address);
        }

        pb.println(format!("{}: icmp_seq={idx} ttl=120 time={time} ms", time_line, idx = index, time = rtt));
        std::thread::sleep(std::time::Duration::from_millis(500));
      }
    }

    // --- bitping.com ping statistics (Ubuntu / United States, New York) ---
    let mut stat_line_1 = format!("--- {endpoint} ping statistics (", endpoint = job.endpoint);
    if let Ok(node_info) = serde_json::from_value::<crate::models::NodeInfo>(jobres.data["nodeInfo"].clone()) {
      stat_line_1 = format!("{}{os} / ", stat_line_1,os = node_info.operating_system)
    }

    let country_code = jobres.location.country_code.clone();
    let emoji = emojis::get_emoji_for_country_code(country_code);
    if config.show_emojis == true && emoji.is_some() == true {
      stat_line_1 = format!("{}{country_code}, {city}) ---", stat_line_1, country_code = console::Emoji(&format!("{} ", &emoji.unwrap()), &jobres.location.country), city = jobres.location.city);
      pb.println(stat_line_1);
    } else {
      stat_line_1 = format!("{}{country}, {city}) ---", stat_line_1, country = jobres.location.country, city = jobres.location.city);
      pb.println(stat_line_1);
    }

    // If we have packet loss stats, show it here
    // 4 packets transmitted, 4 packets received, 0.0% packet loss
    if let Some(packet_loss) = jobres.data.get("packet_loss"){
      let stat_line_2 = format!("{attempts} packets transmitted, {received} packets received, {packet_loss}% packet loss", attempts = jobres.data["attempts"], received = jobres.data["packets_recv"], packet_loss = packet_loss);
      pb.println(stat_line_2);
    }

    // round-trip min/avg/max/stddev = 10.127/13.020/16.181/2.898 ms
    let mut stats_line = String::from("");
    let mut which_stats = String::from("");
    if let Some(min) = jobres.data.get("min") {
      which_stats = format!("min/");
      stats_line = format!("{}{min}/", stats_line, min = min)
    }

    let avg = jobres.response_time.clone();
    which_stats = format!("{}avg", which_stats);
    stats_line = format!("{}{avg}", stats_line, avg = avg);

    if let Some(max) = jobres.data.get("max") {
      which_stats = format!("{}/max", which_stats);
      stats_line = format!("{}/{max}", stats_line, max = max)
    }
    if let Some(std_dev) = jobres.data.get("std_dev") {
      which_stats = format!("{}/stddev", which_stats);
      stats_line = format!("{}/{std_dev}", stats_line, std_dev = std_dev)
    }
    pb.println(format!("round-trip {} = {} ms", which_stats, stats_line))
  }
}

/**
 * PING asdasdasd.com (199.59.242.153): 56 data bytes
Request timeout for icmp_seq 0
Request timeout for icmp_seq 1
Request timeout for icmp_seq 2
Request timeout for icmp_seq 3
^C
--- asdasdasd.com ping statistics ---
5 packets transmitted, 0 packets received, 100.0% packet loss
 */
pub fn display_failed_ping(pb: &indicatif::ProgressBar, job: &crate::models::GetJobAPIResponse) {
  let config = app_config::get_configuration();

  for jobres in &job.job_responses {
    // PING bitping.com (76.76.21.21): 56 data bytes
    let mut line_1 = format!("\nPING"); 

    if config.diagnostics.show_ping_type == true {
      if let Some(ping_type) = jobres.data.get("pingType") {
        line_1 = format!("{} ({type})", line_1, type = ping_type);
      }
    }
    
    line_1 = format!("{} {endpoint}", line_1, endpoint = job.endpoint);
    if let Some(ip_address) = jobres.data.get("ip_address") {
      line_1 = format!("{} ({ip_address})", line_1, ip_address = ip_address);
    }
    line_1 = format!("{}: 56 data bytes", line_1);
    pb.println(line_1);

    // Request timeout for icmp_seq 0
    // if let Some(attempts_val) = jobres.data.get("attempts") {
      let attempts = 3;

      for index in 0..attempts {
        pb.println(format!("Request timeout for icmp_seq {idx}", idx = index));
        std::thread::sleep(std::time::Duration::from_millis(500));
      }
    // }

    // --- bitping.com ping statistics ---
    let mut stat_line_1 = format!("--- {endpoint} ping statistics (", endpoint = job.endpoint);
    if let Ok(node_info) = serde_json::from_value::<crate::models::NodeInfo>(jobres.data["nodeInfo"].clone()) {
      stat_line_1 = format!("{}{os} / ", stat_line_1,os = node_info.operating_system)
    }

    let country_code = jobres.location.country_code.clone();
    let emoji = emojis::get_emoji_for_country_code(country_code);
    if config.show_emojis == true && emoji.is_some() == true {
      stat_line_1 = format!("{}{country_code}, {city}) ---", stat_line_1, country_code = console::Emoji(&format!("{} ", &emoji.unwrap()), &jobres.location.country), city = jobres.location.city);
      pb.println(stat_line_1);
    } else {
      stat_line_1 = format!("{}{country}, {city}) ---", stat_line_1, country = jobres.location.country, city = jobres.location.city);
      pb.println(stat_line_1);
    }


    // If we have packet loss stats, show it here
    // 4 packets transmitted, 4 packets received, 0.0% packet loss
    if let Some(packet_loss) = jobres.data.get("packet_loss") {
      let stat_line_2 = format!("{attempts} packets transmitted, {received} packets received, {packet_loss}% packet loss", attempts = jobres.data["attempts"], received = jobres.data["packets_recv"], packet_loss = packet_loss);
      pb.println(stat_line_2);
    } else {
      let error_string = format!("3 packets transmitted, 0 packets recieved, 100% packet loss");
      pb.println(format!("{}", error_string.color(Color::Red)));
    }
  }
}