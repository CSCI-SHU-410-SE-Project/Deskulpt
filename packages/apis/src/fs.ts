import { invoke } from "@tauri-apps/api/core";

function exists(widgetId: string, path: string) {
  return invoke<boolean>("plugin:apis-fs|exists", { widgetId, path });
}

function isFile(widgetId: string, path: string) {
  return invoke<boolean>("plugin:apis-fs|is_file", { widgetId, path });
}

function readFile(widgetId: string, path: string) {
  return invoke<string>("plugin:apis-fs|read_file", { widgetId, path });
}

function writeFile(widgetId: string, path: string, content: string) {
  return invoke<void>("plugin:apis-fs|write_file", { widgetId, path, content });
}

export { exists, isFile, readFile, writeFile };
