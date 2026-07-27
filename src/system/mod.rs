pub mod cpu;
pub mod memory;
pub mod network;
pub mod disks;
pub mod gpu;

use sysinfo::{System, Networks, Disks};

pub struct SystemMonitor {
    pub sys: System,
    pub networks: Networks,
    pub disks: Disks,
    pub gpu: gpu::GpuMonitor,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        
        let mut networks = Networks::new_with_refreshed_list();
        let mut disks = Disks::new_with_refreshed_list();
        
        let gpu = gpu::GpuMonitor::new();
        
        Self {
            sys,
            networks,
            disks,
            gpu,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.networks.refresh(true);
        self.disks.refresh(true);
        self.gpu.refresh();
    }
}
