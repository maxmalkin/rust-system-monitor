use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Result, Write};
use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, DiskExt, NetworkExt, ProcessExt, System, SystemExt};

#[derive(Parser, Debug)]
#[command(name = "rust-system-monitor")]
#[command(about = "Simple system monitor")]
#[command(version)]
struct Config {

	// update interval in millis
    #[arg(short, long, default_value = "200")]
    interval: u64;

	// dont show network usage
	#[arg(long)]
	no_network: bool;

	// dont show disk usage
	#[arg(long)]
	no_disk: bool;
}

fn main() -> Result<()> {
    let mut sys = System::new_all();
    let mut stdout = stdout();

    // hide cursor and clear screen
    execute!(stdout, Hide, Clear(ClearType::All))?;

    // infinite loop to allow realtime updates
    loop {
        sys.refresh_all();

        // move cursor to start
        execute!(stdout, MoveTo(0, 0))?; // ? used to throw an error

        println!("--- SYSTEM MONITOR RUNNING ---");
        println!("\r"); // ensure full overwrite of previous line

        // format usage to one decimal place
        println!(
            "  CPU Usage:    {:>6.1}%",
            sys.global_cpu_info().cpu_usage()
        );
        let used_mem = sys.used_memory() / 1024 / 1024;
        let total_mem = sys.total_memory() / 1024 / 1024;
        // cast memory to float to calculate percentage accurately
        let mem_used_percent = (used_mem as f64 / total_mem as f64) * 100.0; // has to be 100.0, cannot multiply f64 by int

        println!(
            "  Memory Usage:       {:>6} MB / {:>6} MB ({:>5.1}%)",
            used_mem, total_mem, mem_used_percent
        );

        // if not disabled by flag
        if !config.no_disk {
	        println!();
	        println!("Disk Usage:");
	        println!(
	            "  {:<20} {:>10} {:>10} {:>8}",
	            "Device", "Used", "Total", "Usage"
	        );
	        println!(
	            "  {:<20} {:>10} {:>10} {:>8}",
	            "------", "----", "-----", "-----"
	        );

	        for disk in sys.disks() {
	            let name = disk.name().to_str().unwrap_or("Unknown");
	            let total_space = disk.total_space() / 1024 / 1024 / 1024; // convert to GB
	            let available_space = disk.available_space() / 1024 / 1024 / 1024;
	            let used_space = total_space - available_space;
	            let used_space_percent = if total_space > 0 {
	                (used_space as f64 / total_space as f64) * 100.0 // cast to f64 to get accurate percentage
	            } else {
	                0.0
	            };

	            println!(
	                "  {:<20} {:>7} GB {:>7} GB {:>6.1}%",
	                name, used_space, total_space, used_space_percent
	            );
	        }
        }

        // if not disabled by flag
	    if !config.no_network {
	        println!();
	        println!("Network Usage:");
	        println!("  {:<15} {:>12} {:>12}", "Interface", "Received", "Sent");
	        println!("  {:<15} {:>12} {:>12}", "---------", "--------", "----");
	        for (name, data) in sys.networks() {
	            println!(
	                "  {:<15} {:>9} KB {:>9} KB",
	                name,
	                data.total_received() / 1024, // convert to KB, since MB is too large
	                data.total_transmitted() / 1024
	            );
	        }
	    }

        println!();
        println!("Most Used (CPU):");
        println!("  {:<25} {:>8} {:>10}", "Process", "CPU", "Memory");
        println!("  {:<25} {:>8} {:>10}", "-------", "---", "------");
        // take all processes and put them into mutable array
        let mut processes: Vec<_> = sys.processes().iter().collect();
        // sort by usage, |a, b| is like a lambda function, partial_cmp compares floats
        processes.sort_by(|a, b| {
            b.1.cpu_usage()
                .partial_cmp(&a.1.cpu_usage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // take top 5 by usage
        // destructure and dereference process tuple with & and _
        for &(_, process) in processes.iter().take(5) {
            // truncate long names
            let name = if process.name().len() > 25 {
                format!("{}...", &process.name()[..22])
            } else {
                process.name().to_string()
            };

            println!(
                "  {:<25} {:>6.1}% {:>7} MB",
                name,
                process.cpu_usage(),
                process.memory() / 1024 / 1024
            );
        }

        stdout.flush()?;
        // update by interval, default 200 ms
        thread::sleep(Duration::from_millis(config.interval));
    }
}
