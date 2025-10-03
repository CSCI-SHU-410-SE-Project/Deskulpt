import { invoke } from "@tauri-apps/api/core";

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

interface GetSystemInfoOutputPayload {
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

function getSystemInfo(id: string) {
  return invoke<GetSystemInfoOutputPayload>(
    "plugin:deskulpt-core|call_plugin",
    {
      plugin: "sys",
      command: "get_system_info",
      id,
    },
  );
}

export { getSystemInfo };
