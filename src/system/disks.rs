use sysinfo::Disks;

pub struct DiskInfo {
    pub name: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
}

pub fn get_disks_info(disks: &Disks) -> Vec<DiskInfo> {
    disks.iter().map(|disk| {
        let name = disk.name().to_string_lossy().to_string();
        let file_system = disk.file_system().to_string_lossy().to_string();
        
        DiskInfo {
            name,
            file_system,
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        }
    }).collect()
}
