import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";

export async function emitSwitchThemeToCanvas() {
  await emitTo("canvas", "switch-theme");
}

export function listenToSwitchTheme(handler: EventCallback<void>) {
  return listen("switch-theme", handler);
}
