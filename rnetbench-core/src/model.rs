use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub server_url: String,
    pub duration_secs: u64,
    pub parallel_streams: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputSample {
    pub timestamp_ms: u64,
    pub mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    pub mbps_avg: f64,
    pub mbps_peak: f64,
    pub total_bytes: u64,
    pub duration_secs: f64,
    pub samples: Vec<ThroughputSample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub download: Option<DownloadResult>,
}
