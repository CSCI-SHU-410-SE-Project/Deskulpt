import { EventCallback, once } from "@tauri-apps/api/event";

export function listenToWindowReadyOnce(handler: EventCallback<never>) {
  return once("window-ready", handler);
}
