pub fn get_progress_bar_text(endpoint: &str, regions: &Vec<String>) -> String {
    let config = app_config::get_configuration();

    let regions_display = match regions.len() {
        0 => {
            let mut world_message = "Worldwide".to_string();
            if config.show_emojis == true {
                if let Some(emoji) = emojis::get_emoji_for_country_code("WORLD".to_string()) {
                    world_message =
                        format!("{}", console::Emoji(&format!("{}", emoji), "Worldwide"))
                }
            }
            world_message
        }
        _ => regions
            .iter()
            .map(|region| {
                let mut region_msg = region.to_owned();

                if config.show_emojis == true {
                    if let Some(emoji) = custom_validators::get_emoji_safe_region_code(region)
                        .and_then(emojis::get_emoji_for_country_code)
                    {
                        let formatted_emoji = match custom_validators::is_continent(region) {
                            true => format!("{}", emoji),
                            false => format!("{} ", emoji),
                        };

                        region_msg = format!("{}", console::Emoji(&formatted_emoji, region))
                    }
                }
                region_msg
            })
            .collect::<Vec<String>>()
            .join(","),
    };

    return format!("Collecting results for {} [{}]", endpoint, regions_display);
}

