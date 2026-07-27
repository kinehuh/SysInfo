use sysinfo::System;

pub struct CpuInfo {
    pub name: String,
    pub global_usage: f32,
    pub cores: Vec<f32>,
}

pub fn get_cpu_info(sys: &System) -> CpuInfo {
    let cpus = sys.cpus();
    let name = if let Some(cpu) = cpus.first() {
        cpu.brand().to_string().trim().to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let global_usage = sys.global_cpu_usage();
    let cores: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();

    CpuInfo {
        name,
        global_usage,
        cores,
    }
}
