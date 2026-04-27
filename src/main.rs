use clap::Parser;
use rnetbench::{
    config::load_server_catalog,
    download::run_download_simple,
    model::TestConfig,
    ping::{choose_best_server, probe_servers},
};

#[derive(Parser)]
struct Cli {
    /// URL del server da cui scaricare; se omesso, il programma sceglie automaticamente
    #[arg(long)]
    server: Option<String>,

    /// Durata del test (secondi)
    #[arg(long, default_value = "10")]
    duration: u64,

    /// Numero di server con ping migliore da considerare per la scelta casuale
    #[arg(long, default_value = "3")]
    top: usize,

    /// Elenca il catalogo server incorporato ed esce
    #[arg(long)]
    list_servers: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.list_servers {
        match load_server_catalog() {
            Ok(servers) => {
                for server in servers {
                    println!("- {} -> {}", server.label(), server.download_url);
                }
            }
            Err(err) => eprintln!("Error loading server catalog: {err}"),
        }
        return;
    }

    let selected_url = if let Some(server_url) = cli.server {
        println!("Using manually selected server: {server_url}");
        server_url
    } else {
        let servers = match load_server_catalog() {
            Ok(servers) => servers,
            Err(err) => {
                eprintln!("Error loading server catalog: {err}");
                return;
            }
        };

        if servers.is_empty() {
            eprintln!("No enabled servers available in the catalog.");
            return;
        }

        println!("Probing {} candidate servers...", servers.len());
        let probes = probe_servers(&servers).await;
        if let Some(selected) = choose_best_server(&probes, cli.top) {
            println!(
                "Selected server: {} - {:.1} ms",
                selected.server.label(),
                selected.latency_ms
            );
            println!("Download URL: {}", selected.server.download_url);
            selected.server.download_url
        } else {
            let fallback = &servers[0];
            eprintln!(
                "Could not probe any server successfully; falling back to {}.",
                fallback.label()
            );
            println!("Download URL: {}", fallback.download_url);
            fallback.download_url.clone()
        }
    };

    let cfg = TestConfig {
        server_url: selected_url,
        duration_secs: cli.duration,
        parallel_streams: 1,
    };

    println!("Running simple download test...");
    match run_download_simple(&cfg).await {
        Ok(result) => {
            println!("Average: {:.2} Mbit/s", result.mbps_avg);
            println!("Peak: {:.2} Mbit/s", result.mbps_peak);
            println!(
                "Downloaded: {:.2} MB",
                result.total_bytes as f64 / 1_000_000.0
            );
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
