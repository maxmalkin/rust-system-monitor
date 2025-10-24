use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    // format usage to one decimal place
    println!("CPU Usage: {:.1}%", sys.global_cpu_info().cpu_usage());

    println!(
        "Memory: {} MB / {} MB",
        sys.used_memory() / 1024 / 1024, // convert to MB
        sys.total_memory() / 1024 / 1024
    )
}
