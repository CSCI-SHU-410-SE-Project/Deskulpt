import { emitTo, EventCallback, listen } from "@tauri-apps/api/event";
import { Appearance } from "../../types/backend";

interface SwitchAppearancePayload {
  appearance: Appearance;
}

export async function emitSwitchAppearanceToCanvas(
  payload: SwitchAppearancePayload,
) {
  await emitTo("canvas", "switch-appearance", payload);
}

export function listenToSwitchAppearance(
  handler: EventCallback<SwitchAppearancePayload>,
) {
  return listen("switch-appearance", handler);
}
