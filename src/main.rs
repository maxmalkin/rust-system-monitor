use std::thread;
use std::time::Duration;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    // infinite loop to allow realtime updates
    loop {
        print!("\x1B[2J\x1B[1;1H");
        sys.refresh_all();

        println!("Monitor Running...");
        println!();

        // format usage to one decimal place
        println!("CPU Usage: {:.1}%", sys.global_cpu_info().cpu_usage());
        let used_mem = sys.used_memory() / 1024 / 1024;
        let total_mem = sys.total_memory() / 1024 / 1024;
        // cast memory to float to calculate percentage accurately
        let mem_used_percent = (used_mem as f64 / total_mem as f64) * 100.0;

        println!(
            "Memory: {} MB / {} MB, {:.1}%",
            used_mem, total_mem, mem_used_percent
        );

        for disk in sys.disks() {
            let name = disk.name().to_str().unwrap_or("Name not found."); // default if name not found
            let total_space = disk.total_space() / 1024 / 1024 / 1024; // convert disk space to GB
            let available_space = disk.available_space() / 1024 / 1024 / 1024;
            let used_space = total_space - available_space;
            if total_space > 0 {
                let used_space_percent = (used_space as f64 / total_space as f64) * 100;
            } else {
                let used_space_percent = 0;
            }

            println!(
                "{}: {} GB / {} GB, {:.1}%",
                name, used_space, total_space, used_space, used_space_percent
            );
        }

        // update every 1 second.
        thread::sleep(Duration::from_secs(1));
    }
}
