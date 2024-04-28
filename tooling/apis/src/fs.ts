import { invoke } from "@tauri-apps/api/core";

function exists(widgetId: string, path: string): Promise<boolean> {
  return invoke("plugin:apis-fs|exists", { widgetId, path });
}

function isFile(widgetId: string, path: string): Promise<boolean> {
  return invoke("plugin:apis-fs|is_file", { widgetId, path });
}

function readFile(widgetId: string, path: string): Promise<string> {
  return invoke("plugin:apis-fs|read_file", { widgetId, path });
}

function writeFile(widgetId: string, path: string, content: string): Promise<void> {
  return invoke("plugin:apis-fs|write_file", { widgetId, path, content });
}

export { exists, isFile, readFile, writeFile };
