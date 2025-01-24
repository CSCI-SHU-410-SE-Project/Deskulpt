import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import { Theme } from "../../types";

interface SwitchThemePayload {
  theme: Theme;
}

export async function emitSwitchThemeToCanvas(payload: SwitchThemePayload) {
  await emitTo("canvas", "switch-theme", payload);
}

export function listenToSwitchTheme(
  handler: EventCallback<SwitchThemePayload>,
) {
  return listen("switch-theme", handler);
}
