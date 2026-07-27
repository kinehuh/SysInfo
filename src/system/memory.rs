use sysinfo::System;

pub struct MemoryInfo {
    pub total_ram: u64,
    pub used_ram: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

pub fn get_memory_info(sys: &System) -> MemoryInfo {
    MemoryInfo {
        total_ram: sys.total_memory(),
        used_ram: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    }
}
