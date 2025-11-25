# Changelog

All notable changes to this project will be documented in this file.

The format is based on **Keep a Changelog**  
and this project adheres to **Semantic Versioning**.

---

## [0.1.1] – 2025-01-22

### 🚀 Added

- Introduced full GitHub Actions workflow:
  - `rust.yml` for building and testing across platforms. 
  - `ci.yml` for packaging, signing, and publishing release artifacts (tar.gz, zip, .deb, checksums, signatures).
- Added complete application icon management pipeline:
  - New high-resolution PNG icon (`rnetbench.png`) in `assets/`
  - Windows executable icon embedding via `build.rs` and `winresource`
  - Multi-size PNG set generated using `generate_icons.*`
  - Automatic inclusion of icons in `.deb` packages via `package.metadata.deb`
- Updated project layout to support cross-platform asset generation.

### 🧹 Improved

- Standardized asset folder structure under `rnetbench-cli/assets/`
- Ensured consistent icon naming and sizing across all platforms.

### 🛠️ Internal

- Added `dev_tools/` submodule with unified ImageMagick utility scripts. 
- Updated `.gitignore` to support the new asset workflow.

---

## [0.1.0] - 2025-02-21

### 🎉 Initial release

This is the first public version of **rNetBench**, a modern and cross-platform
network benchmarking tool written in Rust.  
It introduces the project structure and the core foundations of the benchmarking engine.

### Added

- Workspace architecture:
    - `rnetbench-core` – core benchmarking library
    - `rnetbench-cli` – command-line interface
    - `dev_tools` submodule (shared scripts)
- Initial download benchmark implementation:
    - Single-stream HTTP download engine
    - Throughput measurement based on byte counters
    - Periodic sampling (1-second interval)
    - Average and peak Mbps calculation
- Basic data models:
    - `TestConfig`
    - `DownloadResult`
    - `ThroughputSample`
    - `TestResult`
- Initial CLI implementation:
    - `--server <URL>`
    - `--duration <seconds>`
- HTTP client implementation with:
    - custom User-Agent
    - automatic HTTP/1.1 / HTTP/2 negotiation
    - redirect handling
    - safe timeout logic
- Basic workspace README

---

## 🚧 Upcoming features

These features are planned for upcoming releases:

- Multi-stream download engine
- Upload benchmark
- Latency and jitter measurements (HTTP or ICMP)
- Config file (TOML)
- JSON output mode
- Local history database
- Self-hosted benchmark server (`rnetbench-server`)
- GitHub Actions CI + multi-platform release artifacts

