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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCatalog {
    pub servers: Vec<ServerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEntry {
    pub id: String,
    pub provider: String,
    pub location: String,
    pub country: String,
    #[serde(default)]
    pub continent: String,
    pub download_url: String,
    /// URL leggero usato solo per misurare la latenza; se assente si usa download_url
    #[serde(default)]
    pub probe_url: Option<String>,
    #[serde(default = "default_server_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ServerProbeResult {
    pub server: ServerEntry,
    pub latency_ms: f64,
}

impl ServerEntry {
    pub fn label(&self) -> String {
        format!("{} ({}, {})", self.provider, self.location, self.country)
    }

    /// URL da usare per il probing della latenza
    pub fn effective_probe_url(&self) -> &str {
        self.probe_url.as_deref().unwrap_or(&self.download_url)
    }
}

fn default_server_enabled() -> bool {
    true
}
