use crate::model::{ServerEntry, ServerProbeResult};
use reqwest::{Client, header};
use std::time::{Duration as StdDuration, SystemTime, UNIX_EPOCH};
use tokio::{task::JoinSet, time::Instant};

const DEFAULT_PROBE_TIMEOUT_MS: u64 = 2_500;

pub async fn probe_servers(servers: &[ServerEntry]) -> Vec<ServerProbeResult> {
    if servers.is_empty() {
        return Vec::new();
    }

    let client = match Client::builder()
        .user_agent("rNetBench/0.2")
        .timeout(StdDuration::from_millis(DEFAULT_PROBE_TIMEOUT_MS))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
    {
        Ok(client) => client,
        Err(_) => return Vec::new(),
    };

    let mut set = JoinSet::new();
    for server in servers.iter().cloned() {
        let client = client.clone();
        set.spawn(async move { probe_server(client, server).await });
    }

    let mut results = Vec::new();
    while let Some(joined) = set.join_next().await {
        if let Ok(Some(result)) = joined {
            results.push(result);
        }
    }

    results.sort_by(|left, right| left.latency_ms.total_cmp(&right.latency_ms));
    results
}

pub fn best_probe_candidates(probes: &[ServerProbeResult], top_n: usize) -> Vec<ServerProbeResult> {
    let mut sorted = probes.to_vec();
    sorted.sort_by(|left, right| left.latency_ms.total_cmp(&right.latency_ms));
    let limit = top_n.max(1).min(sorted.len());
    sorted.into_iter().take(limit).collect()
}

pub fn choose_best_server(probes: &[ServerProbeResult], top_n: usize) -> Option<ServerProbeResult> {
    let candidates = best_probe_candidates(probes, top_n);
    if candidates.is_empty() {
        return None;
    }

    let index = pseudo_random_index(candidates.len());
    candidates.get(index).cloned()
}

async fn probe_server(client: Client, server: ServerEntry) -> Option<ServerProbeResult> {
    let probe_url = server.effective_probe_url().to_owned();
    let latency = match timed_request(client.head(&probe_url)).await {
        Some(latency) => latency,
        None => timed_request(client.get(&probe_url).header(header::RANGE, "bytes=0-0")).await?,
    };

    Some(ServerProbeResult {
        server,
        latency_ms: latency.as_secs_f64() * 1000.0,
    })
}

async fn timed_request(request: reqwest::RequestBuilder) -> Option<StdDuration> {
    let start = Instant::now();
    let response = request.send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }

    Some(start.elapsed())
}

fn pseudo_random_index(len: usize) -> usize {
    if len <= 1 {
        return 0;
    }

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| StdDuration::from_nanos(0))
        .as_nanos() as usize;

    seed % len
}

#[cfg(test)]
mod tests {
    use super::{best_probe_candidates, choose_best_server};
    use crate::model::{ServerEntry, ServerProbeResult};

    fn server(id: &str, latency_ms: f64) -> ServerProbeResult {
        ServerProbeResult {
            server: ServerEntry {
                id: id.to_string(),
                provider: "Provider".to_string(),
                location: id.to_string(),
                country: "XX".to_string(),
                continent: "EU".to_string(),
                download_url: format!("https://example.com/{id}.bin"),
                probe_url: None,
                enabled: true,
            },
            latency_ms,
        }
    }

    #[test]
    fn returns_only_top_n_candidates() {
        let probes = vec![server("a", 50.0), server("b", 10.0), server("c", 20.0)];
        let best = best_probe_candidates(&probes, 2);
        assert_eq!(best.len(), 2);
        assert_eq!(best[0].server.id, "b");
        assert_eq!(best[1].server.id, "c");
    }

    #[test]
    fn chosen_server_is_within_best_candidates() {
        let probes = vec![server("b", 10.0), server("c", 20.0), server("a", 50.0)];
        let best = best_probe_candidates(&probes, 2);
        let chosen = choose_best_server(&probes, 2).expect("a server should be chosen");
        assert!(best.iter().any(|entry| entry.server.id == chosen.server.id));
    }
}
