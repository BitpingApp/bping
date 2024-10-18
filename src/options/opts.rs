use bpaf::{long, OptionParser, Parser};
use color_eyre::eyre;
use keshvar::Continent;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Debug, Clone)]
pub struct Opts {
    pub endpoint: String,
    pub regions: Vec<EarthRegion>,
    pub count: usize,
    pub attempts: usize,
    pub api_key: String,
}

impl Opts {
    pub fn parser() -> OptionParser<Self> {
        let endpoint = bpaf::positional("endpoint")
            .help("Specifies the endpoint (without http://) to ping. eg. bitping.com")
            .guard(|s: &String| !s.is_empty(), "Endpoint cannot be empty");

        let regions = bpaf::long("regions")
                .short('r')
                .help(r#"Specifies the ISO 3166-1 country codes (alpha-2 or alpha-2) & continent names to send jobs to. Defaults to Anywhere.
            (eg. bping -r "AU,CHN,North America" bitping.com)"#)
            .argument::<String>("regions")
            .optional()
            .map(|r| match r {
                Some(v) => parse_alpha_codes(&v).unwrap_or_else(|_e| vec![EarthRegion::Anywhere]),
                None =>vec![EarthRegion::Anywhere]
            });

        let count = bpaf::long("count")
            .short('c')
            .help("Specifies the number of ICMP packets to send per country. Defaults to 3.")
            .argument::<usize>("count")
            .fallback(3);

        let attempts = bpaf::long("attempts")
            .short('a')
            .help("Specifies the number of ping attempts per country. Defaults to 1.")
            .argument::<usize>("attempts")
            .fallback(1);

        let api_key = bpaf::long("api-key")
            .help("Specifies the API key for authentication. Can also be set using the BITPING_API_KEY environment variable.")
            .env("BITPING_API_KEY")
            .argument("api_key");

        bpaf::construct!(Opts {
            regions,
            count,
            attempts,
            api_key,
            endpoint,
        })
        .to_options()
        .descr("A command line utility to ping a website from anywhere in the world!")
        .version(env!("CARGO_PKG_VERSION"))
    }
}

#[derive(Clone, Debug)]
pub enum EarthRegion {
    Continent(keshvar::Continent),
    Country(keshvar::Country),
    Anywhere,
}

impl std::fmt::Display for EarthRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EarthRegion::Continent(continent) => write!(f, "{}", continent.to_string()),
            EarthRegion::Country(country) => write!(f, "{}", country.alpha3().to_string()),
            EarthRegion::Anywhere => write!(f, ""),
        }
    }
}

pub fn parse_alpha_codes(regions: &str) -> eyre::Result<Vec<EarthRegion>> {
    if regions.trim().to_lowercase() == "anywhere" {
        return Ok(vec![EarthRegion::Anywhere]);
    }

    let parts = regions.split(',');

    let mut regions = vec![];

    for region_part in parts {
        if let Ok(alpha2) = keshvar::Alpha2::try_from(region_part) {
            regions.push(EarthRegion::Country(alpha2.to_country()));
            continue;
        }

        if let Ok(alpha3) = keshvar::Alpha3::try_from(region_part) {
            regions.push(EarthRegion::Country(alpha3.to_country()));
            continue;
        }

        if let Ok(continent) = keshvar::Continent::try_from(region_part) {
            regions.push(EarthRegion::Continent(continent));
            continue;
        }

        match region_part.to_lowercase().as_str() {
            "north america" => {
                regions.push(EarthRegion::Continent(Continent::NorthAmerica));
                continue;
            }
            "south america" => {
                regions.push(EarthRegion::Continent(Continent::SouthAmerica));
                continue;
            }
            "america" => {
                println!("Assuming North and South America.");
                regions.extend_from_slice(&[
                    EarthRegion::Continent(Continent::NorthAmerica),
                    EarthRegion::Continent(Continent::SouthAmerica),
                ]);
                continue;
            }
            _ => {}
        }

        tracing::warn!("Unable to identify region '{region_part}'. Skipping.");
    }

    if regions.is_empty() {
        tracing::warn!("No valid regions specified. Defaulting to Anywhere.");
        regions.push(EarthRegion::Anywhere);
    }

    Ok(regions)
}
