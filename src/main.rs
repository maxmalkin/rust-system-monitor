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
        println!(
            "Memory: {} MB / {} MB",
            sys.used_memory() / 1024 / 1024, // convert to MB
            sys.total_memory() / 1024 / 1024
        )
    }
}
