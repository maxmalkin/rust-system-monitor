# Rust System Monitor

Terminal-based system monitor with live metrics and historical tracking.

## Features

- Real-time CPU/memory usage with trend indicators
- Disk and network I/O statistics
- Top 5 processes by CPU usage
- Rolling averages and peaks (last 10 data points)

## Usage

```bash
cargo run                          # 200ms updates (default)
cargo run -- --interval 1000       # custom interval
cargo run -- --no-disk             # hide disk stats
cargo run -- --no-network          # hide network stats
```

## Architecture

```
src/
├── main.rs       - main loop orchestration
├── config.rs     - CLI argument parsing
├── monitor.rs    - system data collection
├── history.rs    - ring buffer for trend tracking
└── display.rs    - terminal rendering
```

**Key patterns:**
- Ring buffer (VecDeque) pre-allocated at startup to avoid mid-loop allocations
- Cursor repositioning instead of screen clearing for flicker-free updates
- Buffered stdout with manual flush control

## Stack

- **sysinfo** - cross-platform system information
- **crossterm** - terminal manipulation
- **clap** - CLI parsing
- **VecDeque** - ring buffer (stdlib)
