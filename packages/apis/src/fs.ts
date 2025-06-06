import { callPlugin } from "./helper";

function appendFile(id: string, payload: { path: string; content: string }) {
  return callPlugin<null>("fs", "append_file", id, payload);
}

function createDir(id: string, payload: { path: string }) {
  return callPlugin<null>("fs", "create_dir", id, payload);
}

function exists(id: string, payload: { path: string }) {
  return callPlugin<boolean>("fs", "exists", id, payload);
}

function isDir(id: string, payload: { path: string }) {
  return callPlugin<boolean>("fs", "is_dir", id, payload);
}

function isFile(id: string, payload: { path: string }) {
  return callPlugin<boolean>("fs", "is_file", id, payload);
}

function readFile(id: string, payload: { path: string }) {
  return callPlugin<string>("fs", "read_file", id, payload);
}

function removeDir(id: string, payload: { path: string }) {
  return callPlugin<null>("fs", "remove_dir", id, payload);
}

function removeFile(id: string, payload: { path: string }) {
  return callPlugin<null>("fs", "remove_file", id, payload);
}

function writeFile(id: string, payload: { path: string; content: string }) {
  return callPlugin<null>("fs", "write_file", id, payload);
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
