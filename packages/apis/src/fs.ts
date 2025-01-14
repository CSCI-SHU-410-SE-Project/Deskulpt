import { invoke } from "@tauri-apps/api/core";

function appendFile(
  widgetId: string,
  payload: { path: string; content: string },
) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "append_file",
    widgetId,
    payload,
  });
}

function createDir(widgetId: string, payload: { path: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "create_dir",
    widgetId,
    payload,
  });
}

function exists(widgetId: string, payload: { path: string }) {
  return invoke<boolean>("call_plugin", {
    plugin: "fs",
    command: "exists",
    widgetId,
    payload,
  });
}

function isDir(widgetId: string, payload: { path: string }) {
  return invoke<boolean>("call_plugin", {
    plugin: "fs",
    command: "is_dir",
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

function removeDir(widgetId: string, payload: { path: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "remove_dir",
    widgetId,
    payload,
  });
}

function removeFile(widgetId: string, payload: { path: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "remove_file",
    widgetId,
    payload,
  });
}

function writeFile(
  widgetId: string,
  payload: { path: string; content: string },
) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "write_file",
    widgetId,
    payload,
  });
}

export {
  appendFile,
  createDir,
  exists,
  isDir,
  isFile,
  readFile,
  removeDir,
  removeFile,
  writeFile,
};
