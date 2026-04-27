<h1 style="text-align: left;">
  <img src="assets/rnetbench.svg" width="90" style="vertical-align: middle; margin-right: 8px;" alt="rNetBench Logo"/>
  rNetBench
</h1>

**rNetBench** is a fast, modern, and cross-platform **network benchmarking tool** written in Rust.  
It provides accurate measurements of download speed (multi-stream planned), upload speed, latency, and jitter — all
through a clean and extensible architecture.

[![Build Status](https://github.com/umpire274/rNetBench/actions/workflows/ci.yml/badge.svg)](https://github.com/umpire274/rNetBench/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/umpire274/rNetBench)](https://github.com/umpire274/rNetBench/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## ✨ Overview

**rNetBench** is designed to offer a reliable and fully open-source alternative to traditional speed-testing tools.  
Its architecture is split into independent components:

- **`rnetbench-core`** → benchmarking library (download/upload/ping engine)
- **`rnetbench-cli`** → command-line interface
- **`dev_tools`** → shared scripts for building, linting, and development

Even in its early versions, the project focuses on:

- reproducible bandwidth measurements
- clean concurrency model using async Rust (Tokio)
- detailed throughput sampling
- modular components suitable for automation and integration

---

## 🆕 News in v0.1.5

- The entire project has been **refactored into a single-crate design** (removed the previous workspace layout).
- All core benchmarking modules have been moved under `src/`.
- CLI entry point unified into a single `main.rs`.
- Improved packaging compatibility for `.deb`, `.zip`, and `.tar.gz` release artifacts.
- Updated CI workflows for multi-platform builds.
- Updated asset structure, including new icons for Windows and Linux desktop environments.

---

## 🚀 Features

### Implemented

- Single-stream **download benchmark**
- Built-in structured server catalog with automatic server selection
- Throughput sampling (periodic)
- Average & peak Mbps calculation
- Custom User-Agent & robust HTTP client setup
- Cross-platform CLI (Windows, macOS, Linux)
- MIT licensed & open source
- Support for packaging into `.deb` + zipped binaries

### Planned

- Multi-stream download benchmark
- Upload benchmark
- Latency & jitter (HTTP + optional ICMP)
- JSON output
- Local result history database
- Self-hosted benchmark server (`rnetbench-server`)
- Plugin-based architecture for test profiles

See the **ROADMAP** and **CHANGELOG.md** for future milestones.

---

## 📦 Installation

### Build from source

```bash
git clone https://github.com/umpire274/rNetBench.git
cd rNetBench
cargo build --release
```

You will find the executable in:

`target/release/rnetbench` (Linux/macOS) or `target\release\rnetbench.exe` (Windows).

### Requirements

- Rust 1.75+ (recommended: latest stable)
- Tokio async runtime
- Rustls TLS backend (no OpenSSL required)

---

## 🛠 Usage

Basic download test with automatic server selection:

```bash
rnetbench --duration 10
```

Manual server override:

```bash
rnetbench --server https://speed.hetzner.de/100MB.bin --duration 10
```

List embedded test servers:

```bash
rnetbench --list-servers
```

Example output:

```
Probing 13 candidate servers...
Selected server: Hetzner (Falkenstein, DE) - 24.7 ms
Download URL: https://fsn1-speed.hetzner.com/10GB.bin
Running simple download test...

=== rNetBench download results ===
Duration:   10.02 s
Downloaded: 52312345 bytes
Average:    41.22 Mbit/s
Peak:       45.87 Mbit/s
```

---

## 📂 Project Structure

```
rNetBench/
├── CHANGELOG.md
├── README.md
├── build.rs
├── Cargo.toml
├── LICENSE
│
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── config.rs
│   ├── download.rs
│   ├── model.rs
│   ├── ping.rs
│   ├── stats.rs
│   ├── upload.rs
│
└── assets/
    ├── rnetbench.svg
    ├── rnetbench.ico
    ├── rnetbench.png
    ├── rnetbench_1024.png
    ├── rnetbench_512.png
    ├── rnetbench_256.png
    ├── rnetbench_128.png
    ├── rnetbench_64.png
    ├── rnetbench_48.png
    ├── rnetbench_32.png
    └── rnetbench_16.png
```

---

## 🧭 Roadmap

**v0.2.0**

- Multi-stream download benchmark
- Improved sampling engine
- Bandwidth saturation logic

**v0.3.0**

- Upload test implementation
- HTTP POST/PUT streaming
- Server capability detection

**v0.4.0**

- Latency & jitter (full reporting, HTTP + optional ICMP)
- CLI improvements
- JSON output mode

**v0.5.0**

- Config file (YAML/TOML)
- Local history database
- Summary statistics

**v1.0.0**

- Self-hosted benchmark server (rnetbench-server)
- Cross-platform installers
- Stable CLI and API
- Complete documentation

---

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome!

Feel free to open an **issue** or submit a **pull request**.

---

## 📜 License

This project is licensed under the **MIT License**.

See the [LICENSE](LICENSE) file for details.

---

## 👤 Author

Developed by **Alessandro Maestri**

GitHub: [@umpire274](https://github.com/umpire274)
