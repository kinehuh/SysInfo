use nvml_wrapper::Nvml;
use serde::Deserialize;
use wmi::WMIConnection;

pub struct GpuInfo {
    pub name: String,
    pub usage: f32,
    pub used_vram: u64,
    pub total_vram: u64,
    pub temperature: u32,
}

pub struct GpuMonitor {
    nvml: Option<Nvml>,
    wmi_con: Option<WMIConnection>,
    pub info: Option<GpuInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32VideoController {
    name: String,
    adapter_ram: Option<u32>,
}

impl GpuMonitor {
    pub fn new() -> Self {
        let nvml = Nvml::init().ok();
        
        let wmi_con = if nvml.is_none() {
            // Setup WMI for fallback
            if let Ok(wmi_con) = WMIConnection::with_namespace_path("ROOT\\CIMV2") {
                Some(wmi_con)
            } else {
                None
            }
        } else {
            None
        };

        Self {
            nvml,
            wmi_con,
            info: None,
        }
    }

    pub fn refresh(&mut self) {
        if let Some(nvml) = &self.nvml {
            if let Ok(device) = nvml.device_by_index(0) {
                let name = device.name().unwrap_or_else(|_| "NVIDIA GPU".to_string());
                let usage = device.utilization_rates().map(|u| u.gpu as f32).unwrap_or(0.0);
                let memory = device.memory_info().ok();
                let temperature = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu).unwrap_or(0);
                
                let (used_vram, total_vram) = if let Some(mem) = memory {
                    (mem.used, mem.total)
                } else {
                    (0, 0)
                };

                self.info = Some(GpuInfo {
                    name,
                    usage,
                    used_vram,
                    total_vram,
                    temperature,
                });
                return;
            }
        }

        if let Some(wmi_con) = &self.wmi_con {
            if let Ok(results) = wmi_con.query::<Win32VideoController>() {
                if let Some(gpu) = results.into_iter().next() {
                    let total_vram = gpu.adapter_ram.unwrap_or(0) as u64;
                    self.info = Some(GpuInfo {
                        name: gpu.name,
                        usage: 0.0, // Hard to get accurately via WMI without PDH
                        used_vram: 0,
                        total_vram,
                        temperature: 0,
                    });
                    return;
                }
            }
        }

        self.info = None;
    }
}
