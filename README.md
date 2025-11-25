<h1 style="text-align: left;">
  <img src="rnetbench-cli/assets/rnetbench.svg" width="90" style="vertical-align: middle; margin-right: 8px;" alt="rNetBench Logo"/>
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

## 🆕 News in version v0.1.2

### 🔧 Fixed

- Replaced **OpenSSL (native-tls)** with **Rustls** in `rnetbench-core` to improve portability and eliminate TLS build failures on
  32-bit Linux (`i686-unknown-linux-gnu`).
- Removed all system-level OpenSSL dependencies during build and runtime.
- Stabilized the multi-platform CI pipeline by removing the need for OpenSSL libraries across all targets.

### 🚀 Improved

- Smaller and more secure binaries thanks to Rustls' memory-safe TLS backend.
- Faster CI builds due to removal of `libssl-dev`, `pkg-config`, and related 32-bit dependencies.
- Updated README requirements to reflect the TLS backend change.

---

## 🆕 News in version v0.1.1

Version **v0.1.1** introduces the first complete automation layer for assets, packaging, and CI/CD.

### ✔ Full icon pipeline

- Added a new SVG/PNG application icon specifically designed for rNetBench
- Automatic generation of multi-size PNG icons (1024 → 16 px) via:
    - `generate_icons.ps1`
    - `generate_icons.sh`
- Automatic generation of Windows `.ico` via:
    - `generate_ico.ps1`
    - `generate_ico.sh`
- Windows executable now embeds the icon through `build.rs` + `winresource`
- Debian packages now include icons under:
  ```bash
  /usr/share/icons/hicolor/<size>/apps/rnetbench.png
  ```

### ✔ New GitHub CI/CD workflows

Two workflows were added under `.github/workflows/`:

- **`rust.yml`** → builds and tests the workspace on all platforms
- **`ci.yml`** → produces release artifacts (ZIP, TAR.GZ, DEB), generates SHA256 checksums and GPG signatures, and
  automatically creates GitHub Releases

### ✔ dev_tools submodule improvements

The `dev_tools` repository now includes:

- Unified ImageMagick wrappers (`magick_tools.*`)
- Cross-platform icon generators (`generate_icons.*`, `generate_ico.*`)
- Parameter naming aligned between PowerShell and Bash (`--input-dir`, `--output-dir`, `--input-file`, `--sizes`,
  `--verbose`, `--magick-path`, `--help`)

### ✔ Other improvements

- Updated project structure to better support asset management
- Ensured consistent PNG naming + output directories
- README and CHANGELOG updated accordingly

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

Basic download test:

```bash
rnetbench-cli --server https://speed.hetzner.de/100MB.bin --duration 10
rnetbench-cli --server https://speed.cloudflare.com/__down?bytes=50000000 --duration 10
```

Example output:

```
Running simple download test against https://speed.cloudflare.com/__down?bytes=50000000 for 10s...

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
├── Cargo.toml
│
├── .github/
│   └── workflows/
│       ├── rust.yml           # Build & test pipeline
│       └── ci.yml             # Packaging, signing & multi-platform releases
│
├── rnetbench-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── model.rs
│       ├── download.rs
│       ├── upload.rs
│       ├── ping.rs
│       ├── stats.rs
│       └── config.rs
│
├── rnetbench-cli/
    ├── Cargo.toml
    ├── build.rs               # Windows icon embed
    └── assets/                # SVG, PNG set, ICO
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

- Latency & jitter (HTTP + optional ICMP)
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
