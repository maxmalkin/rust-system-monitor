use crate::config::Config;
use crate::history::HistoryTracker;
use crate::monitor::SystemData;
use std::io::{Result, Write};
use sysinfo::{DiskExt, NetworkExt, ProcessExt, System, SystemExt};

pub fn display_header() {
    println!("--- SYSTEM MONITOR RUNNING ---");
    println!("\r");
}

pub fn display_cpu_info(data: &SystemData, history: &HistoryTracker) {
    if history.has_data() {
        let cpu_trend = history.cpu_trend();
        let cpu_avg = history.cpu_avg().unwrap_or(0.0);
        let cpu_max = history.cpu_max().unwrap_or(0.0);

        println!("  CPU Usage:    {:>6.1}%, {}", data.cpu_usage, cpu_trend,);
        println!("avg: {:.1}%, peak: {:.1}%)", cpu_avg, cpu_max);
    } else {
        println!("  CPU Usage:    {:>6.1}%", data.cpu_usage);
    }
}

pub fn display_memory_info(data: &SystemData, history: &HistoryTracker) {
    let mem_trend = history.mem_trend();
    println!(
        "  Memory Usage:       {:>6} MB / {:>6} MB ({:>5.1}%), {}",
        data.mem_used_mb, data.total_mem_mb, data.mem_percent, mem_trend
    );

    if history.has_data() {
        let mem_avg = history.mem_avg().unwrap_or(0.0);
        let mem_max = history.mem_max().unwrap_or(0.0);
        println!("avg: {:.1}%, peak: {:.1}%)", mem_avg, mem_max);
    }
}

pub fn display_disk_info(sys: &System) {
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
        let total_space = disk.total_space() / 1024 / 1024 / 1024;
        let available_space = disk.available_space() / 1024 / 1024 / 1024;
        let used_space = total_space - available_space;
        let used_space_percent = if total_space > 0 {
            (used_space as f64 / total_space as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "  {:<20} {:>7} GB {:>7} GB {:>6.1}%",
            name, used_space, total_space, used_space_percent
        );
    }
}

pub fn display_network_info(sys: &System) {
    println!();
    println!("Network Usage:");
    println!("  {:<15} {:>12} {:>12}", "Interface", "Received", "Sent");
    println!("  {:<15} {:>12} {:>12}", "---------", "--------", "----");

    for (name, data) in sys.networks() {
        println!(
            "  {:<15} {:>9} KB {:>9} KB",
            name,
            data.total_received() / 1024,
            data.total_transmitted() / 1024
        );
    }
}

pub fn display_process_info(sys: &System) {
    println!();
    println!("Most Used (CPU):");
    println!("  {:<25} {:>8} {:>10}", "Process", "CPU", "Memory");
    println!("  {:<25} {:>8} {:>10}", "-------", "---", "------");

    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| {
        b.1.cpu_usage()
            .partial_cmp(&a.1.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for &(_, process) in processes.iter().take(5) {
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
}
