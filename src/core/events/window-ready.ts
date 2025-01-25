import { EventCallback, once } from "@tauri-apps/api/event";

export function listenToWindowReadyOnce(handler: EventCallback<void>) {
  return once("window-ready", handler);
}
