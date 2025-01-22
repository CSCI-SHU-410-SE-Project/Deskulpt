import { invoke } from "@tauri-apps/api/core";

function appendFile(id: string, payload: { path: string; content: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "append_file",
    id,
    payload,
  });
}

function createDir(id: string, payload: { path: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "create_dir",
    id,
    payload,
  });
}

function exists(id: string, payload: { path: string }) {
  return invoke<boolean>("call_plugin", {
    plugin: "fs",
    command: "exists",
    id,
    payload,
  });
}

function isDir(id: string, payload: { path: string }) {
  return invoke<boolean>("call_plugin", {
    plugin: "fs",
    command: "is_dir",
    id,
    payload,
  });
}

function isFile(id: string, payload: { path: string }) {
  return invoke<boolean>("call_plugin", {
    plugin: "fs",
    command: "is_file",
    id,
    payload,
  });
}

function readFile(id: string, payload: { path: string }) {
  return invoke<string>("call_plugin", {
    plugin: "fs",
    command: "read_file",
    id,
    payload,
  });
}

function removeDir(id: string, payload: { path: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "remove_dir",
    id,
    payload,
  });
}

function removeFile(id: string, payload: { path: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "remove_file",
    id,
    payload,
  });
}

function writeFile(id: string, payload: { path: string; content: string }) {
  return invoke<void>("call_plugin", {
    plugin: "fs",
    command: "write_file",
    id,
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
