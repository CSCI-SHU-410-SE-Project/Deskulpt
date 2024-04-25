import { invoke } from "@tauri-apps/api";

function exists(widgetId: string, path: string): Promise<boolean> {
  return invoke("plugin:widget_api.fs|exists", { widgetId, path });
}

function isFile(widgetId: string, path: string): Promise<boolean> {
  return invoke("plugin:widget_api.fs|is_file", { widgetId, path });
}

function readFile(widgetId: string, path: string): Promise<string> {
  return invoke("plugin:widget_api.fs|read_file", { widgetId, path });
}

function writeFile(widgetId: string, path: string, content: string): Promise<void> {
  return invoke("plugin:widget_api.fs|write_file", { widgetId, path, content });
}

export { exists, isFile, readFile, writeFile };
