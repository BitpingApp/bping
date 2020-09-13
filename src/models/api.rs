use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobAPIRequest {
  pub job_type: String,
  pub endpoint: String,
  pub regions: Vec<String>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateJobAPIResponse {
  pub endpoint: String,
  pub job_type: String,
  pub regions: Vec<String>,
  pub id: String,
  pub status: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetJobAPIResponse {
    pub id: String,
    pub endpoint: String,
    pub status: String,
    pub job_type: String,
    pub regions: Vec<String>,
    pub configuration: ::serde_json::Value,
    pub price: String,
    pub retry_attempts: String,
    pub expiry: i64,
    pub created: i64,
    pub modified: i64,
    pub version: i64,
    pub job_responses: Vec<JobResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobResponse {
    pub data: serde_json::Value,
    pub response_time: String,
    pub monitor_type: String,
    pub completed: i64,
    pub modified: i64,
    pub version: i64,
    pub location: Location
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "continent")]
    pub continent: String,

    #[serde(rename = "continentCode")]
    pub continent_code: String,

    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "countryCode")]
    pub country_code: String,

    #[serde(rename = "region")]
    pub region: String,

    #[serde(rename = "regionName")]
    pub region_name: String,

    #[serde(rename = "city")]
    pub city: String,

    #[serde(rename = "district")]
    pub district: String,

    #[serde(rename = "zip")]
    pub zip: String,

    #[serde(rename = "lat")]
    pub lat: f64,

    #[serde(rename = "lon")]
    pub lon: f64,

    #[serde(rename = "timezone")]
    pub timezone: String,

    #[serde(rename = "currency")]
    pub currency: String,

    #[serde(rename = "isp")]
    pub isp: String,

    #[serde(rename = "org")]
    pub org: String,

    #[serde(rename = "as")]
    pub location_as: String,

    #[serde(rename = "asname")]
    pub asname: String,

    #[serde(rename = "mobile")]
    pub mobile: bool,

    #[serde(rename = "proxy")]
    pub proxy: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub goos: String,
    pub goarch: String,
    pub is_root: bool,
    pub kernel: String,
    pub version: String,
    pub language: String,
    pub timezone: String,
    pub git_commit: String,
    pub machine_id: String,
    pub architecture: String,
    pub kernel_version: String,
    pub application_name: String,
    pub is_containerized: bool,
    pub operating_system: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub handle: String,
    pub usd_balance: String,
    pub created: String,
    pub modified: String,
    pub referral: String,
}
