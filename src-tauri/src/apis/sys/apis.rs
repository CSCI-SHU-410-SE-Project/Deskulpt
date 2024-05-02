use crate::{apis::utils, commands::CommandOut};
use serde::{Deserialize, Serialize};
use sysinfo::{Disks, Networks, System};
use tauri::{command, AppHandle, Runtime};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub total_swap: u64,
    pub used_swap: u64,
    pub system_name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub host_name: Option<String>,
    pub cpu_count: usize,
    pub cpu_info: Vec<CpuInfo>,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetworkInfo>,
    pub total_memory: u64,
    pub used_memory: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub vendor_id: String,
    pub brand: String,
    pub frequency: u64,
    pub total_cpu_usage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub available_space: u64,
    pub total_space: u64,
    pub mount_point: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interface_name: String,
    pub total_received: u64,
    pub total_transmitted: u64,
}

#[command]
pub(crate) fn get_system_info<R: Runtime>(
    app_handle: AppHandle<R>,
) -> CommandOut<SystemInfo> {
    utils::get_widget_base(&app_handle);
    Ok(get_system())
}

fn get_system() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    let disks_info: Vec<DiskInfo> = Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            available_space: disk.available_space(),
            total_space: disk.total_space(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
        })
        .collect();
    let networks_info: Vec<NetworkInfo> = Networks::new_with_refreshed_list()
        .iter()
        .map(|(interface_name, data)| NetworkInfo {
            interface_name: interface_name.clone(),
            total_received: data.total_received(),
            total_transmitted: data.total_transmitted(),
        })
        .collect();

    let cpu_count = sys.cpus().len();
    let mut cpu_info = Vec::with_capacity(cpu_count);
    for cpu in sys.cpus() {
        cpu_info.push(CpuInfo {
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
            total_cpu_usage: cpu.cpu_usage(),
        });
    }

    SystemInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        system_name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        host_name: System::host_name(),
        cpu_count,
        cpu_info,
        disks: disks_info,
        networks: networks_info,
    }
}
