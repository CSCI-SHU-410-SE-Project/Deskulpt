import { EventCallback, once } from "@tauri-apps/api/event";

export function listenToExitAppOnce(handler: EventCallback<never>) {
  return once("exit-app", handler);
}
