use crate::model::{DownloadResult, TestConfig, ThroughputSample};
use reqwest::Client;
use std::error::Error;
use tokio::time::{Duration, Instant, sleep};

pub async fn run_download_simple(
    cfg: &TestConfig,
) -> Result<DownloadResult, Box<dyn Error + Send + Sync>> {
    let client = Client::builder()
        .user_agent("rNetBench/0.1")
        .timeout(Duration::from_secs(cfg.duration_secs + 5))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    let mut resp = client
        .get(&cfg.server_url)
        .send()
        .await?
        .error_for_status()?;

    let start = Instant::now();
    let mut total_bytes: u64 = 0;
    let mut samples = Vec::new();

    let mut last_sample_time = Instant::now();

    loop {
        tokio::select! {
            chunk = resp.chunk() => {
                let chunk = chunk?;
                if let Some(c) = chunk {
                    total_bytes += c.len() as u64;
                } else {
                    // Stream terminato dal server
                    break;
                }
            }
            _ = sleep(Duration::from_millis(100)) => {
                let elapsed_ms = last_sample_time.elapsed().as_millis() as u64;
                if elapsed_ms >= 1000 {
                    let secs = start.elapsed().as_secs_f64();
                    if secs > 0.0 {
                        let mbps = (total_bytes as f64 * 8.0 / 1_000_000.0) / secs;
                        samples.push(ThroughputSample {
                            timestamp_ms: (secs * 1000.0) as u64,
                            mbps,
                        });
                    }
                    last_sample_time = Instant::now();
                }
            }
        }

        if start.elapsed().as_secs() >= cfg.duration_secs {
            break;
        }
    }

    let secs = start.elapsed().as_secs_f64().max(0.000_001);
    let mbps_avg = (total_bytes as f64 * 8.0 / 1_000_000.0) / secs;
    if samples.is_empty() {
        samples.push(ThroughputSample {
            timestamp_ms: (secs * 1000.0) as u64,
            mbps: mbps_avg,
        });
    }
    let mbps_peak = samples.iter().map(|s| s.mbps).fold(mbps_avg, f64::max);

    Ok(DownloadResult {
        mbps_avg,
        mbps_peak,
        total_bytes,
        duration_secs: secs,
        samples,
    })
}
