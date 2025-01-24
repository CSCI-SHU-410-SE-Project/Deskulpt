import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";

interface BatchRemovePayload {
  ids: string[];
}

export async function emitBatchRemoveToCanvas(payload: BatchRemovePayload) {
  await emitTo("canvas", "batch-remove", payload);
}

export function listenToBatchRemove(
  handler: EventCallback<BatchRemovePayload>,
) {
  return listen("batch-remove", handler);
}
