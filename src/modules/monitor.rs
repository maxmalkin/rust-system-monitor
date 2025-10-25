use crate::modules::history::History;
use sysinfo::{CpuExt, System, SystemExt};

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

pub fn system_data_to_history(data: &SystemData) -> History {
    History {
        cpu_usage: data.cpu_usage,
        mem_used_mb: data.mem_used_mb,
        mem_percent_usage: data.mem_percent,
    }
}
