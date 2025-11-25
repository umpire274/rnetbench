use clap::Parser;
use rnetbench::{download::run_download_simple, model::TestConfig};

#[derive(Parser)]
struct Cli {
    /// URL del server da cui scaricare
    #[arg(long)]
    server: String,

    /// Durata del test (secondi)
    #[arg(long, default_value = "10")]
    duration: u64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let cfg = TestConfig {
        server_url: cli.server,
        duration_secs: cli.duration,
        parallel_streams: 1,
    };

    println!("Running simple download test...");
    match run_download_simple(&cfg).await {
        Ok(result) => {
            println!("Average: {:.2} Mbit/s", result.mbps_avg);
            println!("Peak: {:.2} Mbit/s", result.mbps_peak);
            println!("Downloaded: {} bytes", result.total_bytes);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
