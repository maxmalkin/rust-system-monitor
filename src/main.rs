mod config;
mod display;
mod history;
mod monitor;
mod rates;

use clap::Parser;
use crossterm::cursor::Hide;
use crossterm::execute;
use std::io::{stdout, Result, Write};
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt};

use config::Config;
use display::*;
use history::HistoryTracker;
use monitor::{collect_system_data, system_data_to_history, collect_network_data, collect_disk_data};
use rates::RateTracker;

fn main() -> Result<()> {
    let config = Config::parse();
    let mut sys = System::new_all();
    let mut stdout = stdout();
    let mut history = HistoryTracker::new(10);
    let mut rate_tracker = RateTracker::new();

    execute!(stdout, Hide)?;

    loop {
        sys.refresh_all();

        print!("\x1B[2J\x1B[1;1H");

        let system_data = collect_system_data(&sys);
        let history_data = system_data_to_history(&system_data);
        history.add(history_data);

        let network_data = collect_network_data(&sys);
        let disk_data = collect_disk_data(&sys);

        let network_rates = rate_tracker.update_network_rates(&network_data);
        let disk_rates = rate_tracker.update_disk_rates(&disk_data);

        display_header();
        display_cpu_info(&system_data, &history)?;
        display_memory_info(&system_data, &history)?;

        if !config.no_disk {
            display_disk_info(&sys)?;
            display_disk_rates(&disk_rates)?;
        }

        if !config.no_network {
            display_network_rates(&network_rates)?;
        }

        display_process_info(&sys);

        stdout.flush()?;
        thread::sleep(Duration::from_millis(config.interval));
    }
}
