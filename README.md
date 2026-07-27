# SysInfo

A fast, lightweight Windows system monitoring dashboard for the terminal. Built in Rust.

## Features
- **Dashboard:** Live metrics for CPU, RAM, Swap, Storage, Network, and Uptime.
- **GPU Stats:** NVIDIA GPU monitoring via NVML, with basic WMI fallback for AMD/Intel.
- **Process Manager:** View, sort, and kill processes interactively.
- **Configurable:** Automatically creates a `sysinfo.toml` on your Desktop.

## Installation

Download the latest executable from the [Releases](https://github.com/kinehuh/SysInfo/releases) page.

### Build from source
```sh
git clone https://github.com/kinehuh/SysInfo.git
cd SysInfo
cargo build --release
```
The compiled binary will be in `target/release/sysinfo-app.exe`.

## License
MIT
