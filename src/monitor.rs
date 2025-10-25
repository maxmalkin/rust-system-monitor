use crate::history::History;
use sysinfo::{CpuExt, System, SystemExt, DiskExt, NetworkExt};
use std::collections::HashMap;

pub struct SystemData {
    pub cpu_usage: f32,
    pub mem_used_mb: u64,
    pub total_mem_mb: u64,
    pub mem_percent: f64,
}

pub fn collect_system_data(sys: &System) -> SystemData {
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let mem_used_mb = sys.used_memory() / 1024 / 1024;
    let total_mem_mb = sys.total_memory() / 1024 / 1024;
    let mem_percent = (mem_used_mb as f64 / total_mem_mb as f64) * 100.0;

    SystemData {
        cpu_usage,
        mem_used_mb,
        total_mem_mb,
        mem_percent,
    }
}

pub fn collect_network_data(sys: &System) -> HashMap<String, (u64, u64)> {
    let mut data = HashMap::new();

    for (name, net_data) in sys.networks() {
        data.insert(
            name.clone(),
            (net_data.total_received(), net_data.total_transmitted()),
        );
    }

    data
}

pub fn collect_disk_data(sys: &System) -> HashMap<String, (u64, u64)> {
    let mut data = HashMap::new();

    for disk in sys.disks() {
    	// disk name as key
    	let name = disk.name().to_str().unwrap_or("Unknown").to_string();

        data.insert(
            name,
            (disk.total_space(), disk.available_space()),
        );
    }

    data
}

pub fn system_data_to_history(data: &SystemData) -> History {
    History {
        cpu_usage: data.cpu_usage,
        mem_used_mb: data.mem_used_mb,
        mem_percent_usage: data.mem_percent,
    }
}
