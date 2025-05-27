import { invoke } from "@tauri-apps/api/core";
import { Decoder, Encoder } from "@msgpack/msgpack";

const encoder = new Encoder();
const decoder = new Decoder();

export async function callPlugin<T>(
  plugin: string,
  command: string,
  id: string,
  payload: unknown,
): Promise<T> {
  const meta = encoder.encode([plugin, command, id]);
  const data = encoder.encode(payload);

  const message = new Uint8Array(meta.byteLength + data.byteLength);
  message.set(meta, 0);
  message.set(data, meta.byteLength);

  const result = await invoke<ArrayBuffer>("call_plugin", message);
  return decoder.decode(result) as T;
}
