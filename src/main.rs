use std::thread;
use std::time::Duration;
use crossterm::{
	cursor::{MoveTo, Hide, Show},
	execute,
	terminal::{Clear, ClearType},
}
use sysinfo::{CpuExt, DiskExt, NetworkExt, ProcessExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    let mut stdout = stdout();

    // hide cursor and clear screen
    execute!(stdout, Hide, Clear(ClearType::All))?;

    // infinite loop to allow realtime updates
    loop {
        sys.refresh_all();

        // move cursor to start
        execute!(stdout, MoveTo(0,0))?; // ? used to throw an error

        println!("Monitor Running...");
        println!("\r"); // ensure full overwrite of previous line

        // format usage to one decimal place
        println!("CPU Usage: {:.1}%", sys.global_cpu_info().cpu_usage());
        let used_mem = sys.used_memory() / 1024 / 1024;
        let total_mem = sys.total_memory() / 1024 / 1024;
        // cast memory to float to calculate percentage accurately
        let mem_used_percent = (used_mem as f64 / total_mem as f64) * 100.0; // has to be 100.0, cannot multiply f64 by int

        println!(
            "Memory: {} MB / {} MB, {:.1}%",
            used_mem, total_mem, mem_used_percent
        );

        println!();
        println!("Disk Usage:");
        for disk in sys.disks() {
            let name = disk.name().to_str().unwrap_or("Name not found."); // default if name not found
            let total_space = disk.total_space() / 1024 / 1024 / 1024; // convert disk space to GB
            let available_space = disk.available_space() / 1024 / 1024 / 1024;
            let used_space = total_space - available_space;
            // equivalent to a ternary
            let used_space_percent = if total_space > 0 {
                (used_space as f64 / total_space as f64) * 100.0
            } else {
                0.0
            };

            println!(
                "{}: {} GB / {} GB, {:.1}%",
                name, used_space, total_space, used_space_percent
            );
        }

        println!();
        println!("Network Usage:");
        for (name, data) in sys.networks() {
            println!(
                "{}: received: {} KB, sent: {} KB",
                name,
                data.total_received() / 1024, // convert to KB, since MB is too large
                data.total_transmitted() / 1024
            );
        }

        println!();
        println!("Most Used (CPU):");
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
            println!(
                "{}: {:.1}% CPU, {} MB RAM",
                process.name(),
                process.cpu_usage(),
                process.memory() / 1024 / 1024
            );
        }

        stdout.flush()?;
        // update every 1 second
        thread::sleep(Duration::from_secs(1));
    }

    execute!(stdout, Show)?; // show cursor again when we exit
    Ok(())
}
