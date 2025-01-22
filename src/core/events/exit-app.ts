import { EventCallback, listen } from "@tauri-apps/api/event";

export function listenToExitApp(handler: EventCallback<never>) {
  return listen("exit-app", handler);
}
