//! This module implements the commands for `fs` in `@deskulpt-test/apis`.

use crate::commands::CommandOut;
use serde::Serialize;
use sysinfo::{Disks, Networks, System};
use tauri::command;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SystemInfo {
    total_swap: u64,
    used_swap: u64,
    system_name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    cpu_count: usize,
    cpu_info: Vec<CpuInfo>,
    disks: Vec<DiskInfo>,
    networks: Vec<NetworkInfo>,
    total_memory: u64,
    used_memory: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CpuInfo {
    vendor_id: String,
    brand: String,
    frequency: u64,
    total_cpu_usage: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiskInfo {
    name: String,
    available_space: u64,
    total_space: u64,
    mount_point: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NetworkInfo {
    interface_name: String,
    total_received: u64,
    total_transmitted: u64,
}

#[command]
pub(crate) fn get_system_info() -> CommandOut<SystemInfo> {
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

    Ok(SystemInfo {
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
    })
}
