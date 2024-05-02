import { invoke } from "@tauri-apps/api/core";
function getSystemInfo(widgetId: string, path: string) {
  return invoke("plugin:apis-sys|get_system_info", {
    widgetId: widgetId,
    path: path,
  });
}

export { getSystemInfo };
