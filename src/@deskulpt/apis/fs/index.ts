import { invoke } from "@tauri-apps/api";

function readFile(widgetId: string, path: string) {
  return invoke("plugin:widget_api.fs|read_file", { widgetId: widgetId, path: path });
}

export { readFile };
