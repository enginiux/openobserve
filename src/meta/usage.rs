use super::StreamType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageData {
    pub event: UsageEvent,
    pub day: u32,
    pub hour: u32,
    pub month: u32,
    pub year: i32,
    pub organization_identifier: String,
    pub request_body: String,
    pub size: f64,
    pub unit: String,
    pub user_email: String,
    pub response_time: f64,
    pub stream_type: StreamType,
    pub num_records: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UsageEvent {
    #[serde(rename = "/_bulk")]
    Bulk,
    #[serde(rename = "/_json")]
    Json,
    #[serde(rename = "/_multi")]
    Multi,
    #[serde(rename = "/traces")]
    Traces,
    #[serde(rename = "/v1/write")]
    Metrics,
    #[serde(rename = "/_search")]
    Search,
    #[serde(rename = "/_around")]
    SearchAround,
    #[serde(rename = "/_values")]
    SearchTopNValues,
    Functions,
    Retention,
    #[serde(rename = "/_kinesis_firehose")]
    KinesisFirehose,
    Syslog,
    EnrichmentTable,
}

impl ToString for UsageEvent {
    fn to_string(&self) -> String {
        match self {
            UsageEvent::Bulk => "/_bulk".to_owned(),
            UsageEvent::Json => "/_json".to_owned(),
            UsageEvent::Multi => "/_multi".to_owned(),
            UsageEvent::Traces => "/traces".to_owned(),
            UsageEvent::Metrics => "/v1/write".to_owned(),
            UsageEvent::Search => "/_search".to_owned(),
            UsageEvent::Functions => "functions".to_owned(),
            UsageEvent::Retention => "data_retention".to_owned(),
            UsageEvent::KinesisFirehose => "_kinesis_firehose".to_owned(),
            UsageEvent::Syslog => "syslog".to_owned(),
            UsageEvent::EnrichmentTable => "enrichment_table".to_owned(),
            UsageEvent::SearchAround => "/_around".to_owned(),
            UsageEvent::SearchTopNValues => "/_values".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct RequestStats {
    pub size: f64,
    pub records: u64,
    pub response_time: f64,
}
impl Default for RequestStats {
    fn default() -> Self {
        Self {
            size: 0.0,
            records: 0,
            response_time: 0.0,
        }
    }
}
