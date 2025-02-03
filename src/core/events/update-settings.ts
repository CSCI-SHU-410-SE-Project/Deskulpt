import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import { WidgetSettings } from "../../types";

interface UpdateSettingsPayload {
  id: string;
  settings: Partial<WidgetSettings>;
}

export async function emitUpdateSettingsToCanvas(
  payload: UpdateSettingsPayload,
) {
  await emitTo("canvas", "update-settings", payload);
}

export async function emitUpdateSettingsToManager(
  payload: UpdateSettingsPayload,
) {
  await emitTo("manager", "update-settings", payload);
}

export function listenToUpdateSettings(
  handler: EventCallback<UpdateSettingsPayload>,
) {
  return listen("update-settings", handler);
}
