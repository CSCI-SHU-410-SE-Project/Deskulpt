import { invoke } from "@tauri-apps/api/core";

interface SystemInfo {
  totalSwap: number;
  usedSwap: number;
  systemName?: string;
  kernelVersion?: string;
  osVersion?: string;
  hostName?: string;
  cpuCount: number;
  cpuInfo: CpuInfo[];
  disks: DiskInfo[];
  networks: NetworkInfo[];
  totalMemory: number;
  usedMemory: number;
}

interface CpuInfo {
  vendorId: string;
  brand: string;
  frequency: number;
  totalCpuUsage: number;
}

interface DiskInfo {
  name: string;
  availableSpace: number;
  totalSpace: number;
  mountPoint: string;
}

interface NetworkInfo {
  interfaceName: string;
  totalReceived: number;
  totalTransmitted: number;
}

function getSystemInfo() {
  return invoke<SystemInfo>("plugin:apis-sys|get_system_info");
}

export { getSystemInfo };
