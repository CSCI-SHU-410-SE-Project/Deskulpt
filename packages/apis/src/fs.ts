import { invoke } from "@tauri-apps/api/core";

function exists(widgetId: string, payload: { path: string }) {
  return invoke<boolean>("call_plugin", {
    plugin: "fs",
    command: "exists",
    widgetId,
    payload,
  });
}

function isFile(widgetId: string, payload: { path: string }) {
  return invoke<boolean>("call_plugin", {
    plugin: "fs",
    command: "is_file",
    widgetId,
    payload,
  });
}

function readFile(widgetId: string, payload: { path: string }) {
  return invoke<string>("call_plugin", {
    plugin: "fs",
    command: "read_file",
    widgetId,
    payload,
  });
}

function writeFile(widgetId: string, payload: { path: string; content: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "write_file",
    widgetId,
    payload,
  });
}

export { exists, isFile, readFile, writeFile };
