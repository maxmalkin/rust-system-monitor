# Rust System Monitor

A basic system monitor in Rust, as a project to learn system level programming and realtime programming.

- `sysinfo` package to fetch system info.
- `std::thread` and `Duration` to manage realtime updates and threads.
- `crossterm` for terminal cursor positioning.
- `std::io` for input and output safety.
- `clap` for CLI arg parsing.

## Usage

1. Clone the repo.
2. Run using `cargo run`.

## Options

```bash
# Update interval in milliseconds
cargo run -- --interval [interval]

# Do not show disk info
cargo run -- --no-disk

# Do not show network info
cargo run -- --no-network

```
