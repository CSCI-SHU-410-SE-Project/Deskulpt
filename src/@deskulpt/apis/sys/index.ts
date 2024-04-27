import { invoke } from "@tauri-apps/api";

function getSystemInfo(widgetId: string, path: string) {
  return invoke("plugin:widget_api.sys|get_system_info", {
    widgetId: widgetId,
    path: path,
  });
}

export { getSystemInfo };
