use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    // infinite loop to allow realtime updates
    loop {
        print!("\x1B[2J\x1B[1;1H");
        sys.refresh_all();

        println!("Monitor Running...");

        // format usage to one decimal place
        println!("CPU Usage: {:.1}%", sys.global_cpu_info().cpu_usage());

        let used_mem = sys.used_memory() / 1024 / 1024;
        let total_mem = sys.total_memory() / 1024 / 1024;
        let mem_used_percent = (used_mem as f64 / total_mem as f64) * 100.0;

        println!(
            "Memory: {} MB / {} MB, {:.1}%",
            used_mem, total_mem, mem_used_percent
        );

        // update every 1 second.
        thread::sleep(Duration::from_secs(1));
    }
}
