import { invoke } from "@tauri-apps/api";

function exists(widgetId: string, path: string) {
  return invoke("plugin:widget_api.fs|exists", { widgetId: widgetId, path: path });
}

function isFile(widgetId: string, path: string) {
  return invoke("plugin:widget_api.fs|is_file", { widgetId: widgetId, path: path });
}

function readFile(widgetId: string, path: string) {
  return invoke("plugin:widget_api.fs|read_file", { widgetId: widgetId, path: path });
}

function writeFile(widgetId: string, path: string, content: string) {
  return invoke("plugin:widget_api.fs|write_file", {
    widgetId: widgetId,
    path: path,
    content: content,
  });
}

export { exists, isFile, readFile, writeFile };
