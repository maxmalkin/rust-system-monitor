use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{Clear, ClearType},
};
use std::collections::VecDeque;
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
    interval: u64,

    // dont show network usage
    #[arg(long)]
    no_network: bool,

    // dont show disk usage
    #[arg(long)]
    no_disk: bool,
}

// shape for tracked data
#[derive(Debug, Clone)]
struct History {
    cpu_usage: f32,
    mem_percent_usage: f64,
    mem_used_mb: u64,
}

struct HistoryTracker {
    data: VecDeque<History>,
    max_size: usize,
}

// funcs on HistoryTracker
impl HistoryTracker {
    // create new instance
    fn new(max_size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    // mutable to be able to update instance of self
    fn add(&mut self, data: History) {
        // if data is full, pop oldest entry
        if (self.data.len() >= self.max_size) {
            self.data.pop_front();
        }
        self.data.push_back(data);
    }

    // average cpu usage over runtime
    fn cpu_avg(&self) -> Option<f32> {
        if self.data.is_empty() {
            // no average without data
            return None;
        }
        let sum: f32 = self.data.iter().map(|x| x.cpu_usage).sum();
        return Some(sum / self.data.len() as f32)
    }

    // max cpu usage over runtime
    fn cpu_max(&self) -> Option<f32> {
        self.data
            .iter()
            .map(|x| x.cpu_usage)
            .max_by(|a, b| a.partial_cmp(b).unwrap());
    }

    // average memory usage over runtime
    fn mem_avg(&self) -> Option<f64> {
        if self.data.is_empty() {
            // no average without data
            return None;
        }
        let sum: f64 = self.data.iter().map(|x|) x.mem_percent_usage).sum();
        return Some(sum / self.data.len() as f64);
    }

    // max memory usage over runtime
    fn mem_max(&self) -> Option<f64> {
    	self.data.iter().map(|x| x.mem_percent_usage).max_by(|a, b| a.partial_cmp(b).unwrap());
    }

    // shows if cpu usage is increasing or decreasing
    fn cpu_trend(&self) -> String {
	    if self.data.len() < 2 {
	    	return "".to_string();
	    }

		let curr = self.data.back().unwrap().cpu_usage;
		let prev = self.data[self.data.len() - 2].cpu_usage;

		if curr > prev + 1.0 {
			return "↗".to_string();
		} else if curr < prev - 1.0 {
			return "↘".to_string();
		} else {
			return "→".to_string();
		}
    }

    // shows if memory usage is increasing or decreasing
    fn mem_trend(&self) -> String {
	    if self.data.len() < 2 {
	    	return "".to_string();
	    }

		let curr = self.data.back().unwrap().mem_percent_usage;
		let prev = self.data[self.data.len() - 2].mem_percent_usage;

		if curr > prev + 1.0 {
			return "↗".to_string();
		} else if curr < prev - 1.0 {
			return "↘".to_string();
		} else {
			return "→".to_string();
		}
    }
}

fn main() -> Result<()> {
    let config = Config::parse();

    let mut sys = System::new_all();
    let mut stdout = stdout();

    let mut history = HistoryTracker::new(10); // keep last 10 data points

    // hide cursor and clear screen
    execute!(stdout, Hide, Clear(ClearType::All))?;

    // infinite loop to allow realtime updates
    loop {
        sys.refresh_all();

        // move cursor to start
        execute!(stdout, MoveTo(0, 0))?; // ? used to throw an error

        let current_cpu_usage = sys.global_cpu_info().cpu_usage();
        let current_mem_usage = sys.used_memory() / 1024 / 1024;
        let total_mem = sys.total_memory() / 1024 / 1024;
        let mem_percent_usage = (current_mem_usage as f64 / total_mem as f64) * 100.0;

        let history = History {
            cpu_usage: current_cpu_usage,
            mem_usage: current_mem_usage,
            mem_percent_usage: mem_percent_usage,
        };
        history.add(data);

        println!("--- SYSTEM MONITOR RUNNING ---");
        println!("\r"); // ensure full overwrite of previous line


        // format usage to one decimal place
        println!(
            "  CPU Usage:    {:>6.1}%, {}",
            sys.global_cpu_info().cpu_usage(),
            cpu_trend,
        );
        if history.data.len() > 0 {
        	let cpu_trend = history.cpu_trend();
        	let cpu_avg = history.cpu_avg().unwrap_or(0.0);
        	let cpu_max = history.cpu_max().unwrap_or(0.0);
         	println!("avg: {:.1}%, peak: {:.1}%)",
	            cpu_avg,
	            cpu_max);
        }

        let used_mem = sys.used_memory() / 1024 / 1024;
        let total_mem = sys.total_memory() / 1024 / 1024;
        // cast memory to float to calculate percentage accurately
        let mem_used_percent = (used_mem as f64 / total_mem as f64) * 100.0; // has to be 100.0, cannot multiply f64 by int
       	let mem_trend = history.mem_trend();
        println!(
            "  Memory Usage:       {:>6} MB / {:>6} MB ({:>5.1}%), {}",
            used_mem, total_mem, mem_used_percent, mem_trend
        );

       	let mem_avg = history.mem_avg().unwrap_or(0.0);
       	let mem_max = history.mem_max().unwrap_or(0.0);
       	println!("avg: {:.1}%, peak: {:.1}%)",
      		mem_avg,
            mem_max);

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
