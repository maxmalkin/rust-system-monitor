use std::io::{Write, Result};
use sysinfo::{System, SystemExt, DiskExt, ProcessExt};
use crossterm::style::{Color, SetForegroundColor, ResetColor};
use crossterm::execute;
use crate::config::Config;
use crate::history::HistoryTracker;
use crate::monitor::SystemData;

const CPU_CRITICAL: f32 = 80.0;
const CPU_WARNING: f32 = 60.0;
const MEM_CRITICAL: f64 = 85.0;
const MEM_WARNING: f64 = 70.0;

fn get_cpu_color(usage: f32) -> Color {
    if usage > CPU_CRITICAL {
        Color::Red
    } else if usage > CPU_WARNING {
        Color::Yellow
    } else {
        Color::Green
    }
}

fn get_memory_color(usage: f64) -> Color {
    if usage > MEM_CRITICAL {
        Color::Red
    } else if usage > MEM_WARNING {
        Color::Yellow
    } else {
        Color::Green
    }
}

fn print_colored_value(value: &str, color: Color) -> Result<()> {
    execute!(std::io::stdout(), SetForegroundColor(color))?;
    print!("{}", value);
    execute!(std::io::stdout(), ResetColor)?;
    Ok(())
}

pub fn display_header() {
    println!("--- SYSTEM MONITOR RUNNING ---");
    println!("\r");
}

pub fn display_cpu_info(data: &SystemData, history: &HistoryTracker) -> Result<()> {
    if history.has_data() {
        let cpu_trend = history.cpu_trend();
        let cpu_avg = history.cpu_avg().unwrap_or(0.0);
        let cpu_max = history.cpu_max().unwrap_or(0.0);

        print!("  CPU Usage:    ");

        let formatted = format!("{:>6.1}%", data.cpu_usage);
        print_colored_value(&formatted, get_cpu_color(data.cpu_usage))?;

        println!(", {}", cpu_trend);
        println!("avg: {:.1}%, peak: {:.1}%)", cpu_avg, cpu_max);
    } else {
        print!("  CPU Usage:    ");
        let formatted = format!("{:>6.1}%", data.cpu_usage);
        print_colored_value(&formatted, get_cpu_color(data.cpu_usage))?;
        println!();
    }

    Ok(())
}

pub fn display_memory_info(data: &SystemData, history: &HistoryTracker) -> Result<()> {
    let mem_trend = history.mem_trend();
    print!("  Memory Usage:       {:>6} MB / {:>6} MB (", data.mem_used_mb, data.total_mem_mb);

    let formatted = format!("{:>5.1}%", data.mem_percent);
    print_colored_value(&formatted, get_memory_color(data.mem_percent))?;

    println!("), {}", mem_trend);

    if history.has_data() {
        let mem_avg = history.mem_avg().unwrap_or(0.0);
        let mem_max = history.mem_max().unwrap_or(0.0);
        println!("avg: {:.1}%, peak: {:.1}%)", mem_avg, mem_max);
    }

    Ok(())
}

pub fn display_disk_info(sys: &System) -> Result<()> {
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

        print!("  {:<20} {:>7} GB {:>7} GB ", name, used_space, total_space);

        let formatted = format!("{:>6.1}%", used_space_percent);
        print_colored_value(&formatted, get_memory_color(used_space_percent))?;
        println!();
    }

    Ok(())
}

pub fn display_network_rates(rates: &[(String, u64, u64)]) -> Result<()> {
    println!();
    println!("Network Usage (per second):");
    println!("  {:<15} {:>12} {:>12}", "Interface", "Download", "Upload");
    println!("  {:<15} {:>12} {:>12}", "---------", "--------", "------");

    if rates.is_empty() {
        println!("  (calculating rates...)");
        return Ok(());
    }

    for (name, recv_rate, sent_rate) in rates {
        println!(
            "  {:<15} {:>9} KB/s {:>9} KB/s",
            name,
            recv_rate / 1024,
            sent_rate / 1024
        );
    }

    Ok(())
}

pub fn display_disk_rates(rates: &[(String, u64, u64)]) -> Result<()> {
    println!();
    println!("Disk I/O (per second):");
    println!("  {:<20} {:>12} {:>12}", "Device", "Read", "Write");
    println!("  {:<20} {:>12} {:>12}", "------", "----", "-----");

    if rates.is_empty() {
        println!("  (calculating rates...)");
        return Ok(());
    }

    for (name, read_rate, write_rate) in rates {
        println!(
            "  {:<20} {:>9} KB/s {:>9} KB/s",
            name,
            read_rate / 1024,
            write_rate / 1024
        );
    }

    Ok(())
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
