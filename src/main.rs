mod config;
mod display;
mod history;
mod monitor;

use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Result, Write};
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt};

use config::Config;
use display::*;
use history::HistoryTracker;
use monitor::{collect_system_data, system_data_to_history};

fn main() -> Result<()> {
    let config = Config::parse();

    let mut sys = System::new_all();
    let mut stdout = stdout();
    let mut history = HistoryTracker::new(10);

    execute!(stdout, Hide, Clear(ClearType::All))?;

    loop {
        sys.refresh_all();
        execute!(stdout, MoveTo(0, 0))?;

        // get system info
        let system_data = collect_system_data(&sys);

        // add to history
        let history_data = system_data_to_history(&system_data);
        history.add(history_data);

        // display everything
        display_header();
        display_cpu_info(&system_data, &history);
        display_memory_info(&system_data, &history);

        if !config.no_disk {
            display_disk_info(&sys);
        }

        if !config.no_network {
            display_network_info(&sys);
        }

        display_process_info(&sys);

        stdout.flush()?;
        thread::sleep(Duration::from_millis(config.interval));
    }
}
