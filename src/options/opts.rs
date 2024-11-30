use std::fmt::Display;

use bpaf::{long, OptionParser, Parser};
use color_eyre::eyre;
use keshvar::Continent;

#[derive(Debug, Clone)]
pub enum NetworkPolicy {
    Allowed,
    Denied,
    Required,
}

impl From<Option<bool>> for NetworkPolicy {
    fn from(_value: Option<bool>) -> Self {
        match _value {
            None => NetworkPolicy::Allowed,
            Some(true) => NetworkPolicy::Required,
            Some(false) => NetworkPolicy::Denied,
        }
    }
}

impl Display for NetworkPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkPolicy::Allowed => write!(f, "ALLOWED"),
            NetworkPolicy::Denied => write!(f, "DENIED"),
            NetworkPolicy::Required => write!(f, "REQUIRED"),
        }
    }
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Debug, Clone)]
pub struct Opts {
    pub endpoint: String,
    pub regions: Vec<EarthRegion>,
    pub count: usize,
    pub attempts: usize,
    pub api_key: String,
    pub concurrency: usize,
    pub residential: NetworkPolicy,
    pub mobile: NetworkPolicy,
    pub proxy: NetworkPolicy,
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

        let concurrency = bpaf::long("concurrency")
            .help("Specifies how many concurrent requests to send at once. Defaults to 100.")
            .argument::<usize>("concurrency")
            .fallback(100);

        let residential = bpaf::long("residential")
            .help("Control residential network usage. --residential=true to require, --residential=false to deny, omit to allow.")
            .argument::<bool>("residential")
            .optional()
            .map(NetworkPolicy::from);

        let mobile = bpaf::long("mobile")
            .help("Control mobile network usage. --mobile=true to require, --mobile=false to deny, omit to allow.")
            .argument::<bool>("mobile")
            .optional()
            .map(NetworkPolicy::from);

        let proxy = bpaf::long("proxy")
            .help("Control proxy network usage. --proxy=true to require, --proxy=false to deny, omit to allow.")
            .argument::<bool>("proxy")
            .optional()
            .map(NetworkPolicy::from);

        bpaf::construct!(Opts {
            regions,
            count,
            attempts,
            concurrency,
            api_key,
            residential,
            mobile,
            proxy,
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
    Country(keshvar::Alpha3),
    Anywhere,
}

impl std::fmt::Display for EarthRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EarthRegion::Continent(continent) => write!(f, "{}", continent.to_string()),
            EarthRegion::Country(country) => write!(f, "{}", country.to_string()),
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
            regions.push(EarthRegion::Country(alpha2.to_country().alpha3()));
            continue;
        }

        if let Ok(alpha3) = keshvar::Alpha3::try_from(region_part) {
            regions.push(EarthRegion::Country(alpha3.to_country().alpha3()));
            continue;
        }

        if let Ok(country) = keshvar::Country::try_from(region_part) {
            regions.push(EarthRegion::Country(country.alpha3()));
            continue;
        }

        if let Ok(continent) = keshvar::Continent::try_from(region_part) {
            regions.push(EarthRegion::Continent(continent));
            continue;
        }

        if let Ok(region) = keshvar::Region::try_from(region_part) {
            let continents = match region {
                keshvar::Region::Africa => vec![EarthRegion::Continent(keshvar::Continent::Africa)],
                keshvar::Region::Americas => vec![
                    EarthRegion::Continent(Continent::NorthAmerica),
                    EarthRegion::Continent(Continent::SouthAmerica),
                ],
                keshvar::Region::Antarctica => vec![EarthRegion::Continent(Continent::Antarctica)],
                keshvar::Region::Asia => vec![EarthRegion::Continent(Continent::Asia)],
                keshvar::Region::Europe => vec![EarthRegion::Continent(Continent::Europe)],
                keshvar::Region::Oceania => vec![EarthRegion::Continent(Continent::Australia)],
            };
            regions.extend_from_slice(&continents);

            continue;
        }

        match region_part.to_lowercase().as_str() {
            "america" => {
                println!("Assuming North and South America.");
                regions.extend_from_slice(&[
                    EarthRegion::Continent(Continent::NorthAmerica),
                    EarthRegion::Continent(Continent::SouthAmerica),
                ]);
            }
            "af" | "africa" => {
                regions.push(EarthRegion::Continent(Continent::Africa));
            }
            "an" | "antarctica" => {
                regions.push(EarthRegion::Continent(Continent::Antarctica));
            }
            "as" | "asia" => {
                regions.push(EarthRegion::Continent(Continent::Asia));
            }
            "eu" | "europe" => {
                regions.push(EarthRegion::Continent(Continent::Europe));
            }
            "na" | "north america" => {
                regions.push(EarthRegion::Continent(Continent::NorthAmerica));
            }
            "oc" | "oceania" => {
                regions.push(EarthRegion::Continent(Continent::Australia));
            }
            "sa" | "south america" => {
                regions.push(EarthRegion::Continent(Continent::SouthAmerica));
            }
            _ => {
                tracing::warn!("Unable to identify region '{region_part}'. Skipping.");
            }
        }

        tracing::warn!("Unable to identify region '{region_part}'. Skipping.");
    }

    if regions.is_empty() {
        tracing::warn!("No valid regions specified. Defaulting to Anywhere.");
        regions.push(EarthRegion::Anywhere);
    }

    Ok(regions)
}
