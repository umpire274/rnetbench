# Changelog

All notable changes to this project will be documented in this file.

The format is based on **Keep a Changelog**  
and this project adheres to **Semantic Versioning**.

---

## [0.2.0] — 2026-04-27

### 🚀 Added

- **Structured server catalog** (`assets/test_servers.json`): replaces the plain-text `test_servers.txt` with a typed
  JSON catalog embedded at compile time via `include_str!`.
- **Automatic server selection**: if `--server` is omitted, the program probes all enabled servers concurrently via HTTP
  latency measurements and picks one at random among the best `N` (controlled by `--top`, default 3).
- **`probe_url` field** on each server entry: dedicated lightweight endpoint used for latency probing, independent from
  the large `download_url` file. Falls back to `download_url` when not set.
- **`continent` field** on each server entry: enables future region-based filtering.
- **`--list-servers` flag**: prints the full embedded catalog and exits.
- **`--top <N>` flag**: controls how many lowest-latency candidates are considered for the random final pick.
- **`--server` is now optional**: manual override is still supported; automatic selection is used when omitted.
- **Global server catalog** with 33 enabled entries spanning 6 continents and 7 providers:
    - **EU** (12): Hetzner ×3, OVHcloud, DataPacket ×3, Virtua.Cloud, Clouvider ×3, Leaseweb
    - **NA** (10): Cloudflare anycast, Hetzner ×2, Virtua.Cloud, DataPacket ×3, Clouvider ×3
    - **SA** (1): DataPacket São Paulo
    - **AS** (7): Hetzner SG, DataPacket ×5 (SG, TYO, SEL, BOM, HKG, DXB), Clouvider SG
    - **OC** (1): DataPacket Sydney
    - **AF** (1): DataPacket Johannesburg
- New data models: `ServerCatalog`, `ServerEntry`, `ServerProbeResult`.

### 🔧 Changed

- `--server` argument changed from required to optional (`Option<String>`).
- `Downloaded` output now expressed in MB with two decimal places (e.g. `5.36 MB`) instead of raw bytes.
- `src/config.rs` implemented: loads and filters the embedded server catalog.
- `src/ping.rs` implemented: concurrent HTTP probing, latency ranking, random selection among top-N candidates.
- Fixed zero `Peak` in very short tests by inserting a synthetic final sample when no periodic samples were collected.
- Server catalog moved to `assets/test_servers.json` (previously attempted in `src/`).

### 🗑️ Removed

- Plain-text `test_servers.txt` superseded by the structured JSON catalog (file kept in repo for reference).
- `ThinkBroadband` server disabled by default (`"enabled": false`) due to HTTP-only endpoint.

---

## [0.1.5] — 2025-XX-XX

### 🔧 Changed

- Refactored the entire project structure: removed the previous workspace (`rnetbench-core` + `rnetbench-cli`) and
  merged all components into a single unified crate.
- Moved all benchmarking modules (`download`, `upload`, `ping`, `stats`, `model`) into the main `src/` directory.
- Consolidated the CLI entry point: `main.rs` is now at `src/main.rs`.
- Updated `Cargo.toml` with a simplified, single-crate configuration including metadata, dependencies, build script, and
  packaging info.
- Updated icons and asset layout under the root-level `assets/` directory.
- Cleaned up outdated workspace configuration and path-based dependencies.
- Adjusted build scripts and CI workflows to work with the new project layout.

### 🗑️ Removed

- Deleted `rnetbench-core/` and `rnetbench-cli/` directories.
- Removed the `[workspace]` section from the root `Cargo.toml`.
- Eliminated path-dependency references previously used for the workspace.

### 🚀 Notes

This refactors significantly simplifies packaging, publishing, CI pipelines, and future release management.  
It also prepares the project for distribution on crates.io and for cleaner multi-platform builds.

---

## [0.1.2] - 2025-11-25

### 🔧 Fixed

- Replaced **OpenSSL (native-tls)** with **Rustls** in `rnetbench-core` to ensure fully portable builds across all
  targets,
  including `i686-unknown-linux-gnu` on GitHub Actions.
- Removed the need for system OpenSSL libraries during CI builds.
- Improved reliability of cross-compilation, especially for 32-bit Linux.
- Reduced dependency footprint and improved security by relying on a memory-safe TLS backend.

### 💡 Technical Notes

-Updated `reqwest` to use:

```toml
default-features = false
features = ["rustls-tls-native-roots"]
```

- Eliminated OpenSSL-specific environment variables, pkg-config lookups, and build-time dependencies.

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
- Region-based server filtering (`--region eu`, `--region us`, …)
- Config file (TOML)
- JSON output mode
- Local history database
- Self-hosted benchmark server (`rnetbench-server`)

