import { EventCallback, once } from "@tauri-apps/api/event";

export function listenToExitAppOnce(handler: EventCallback<void>) {
  return once("exit-app", handler);
}
