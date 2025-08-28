import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import { WidgetSettings } from "../bindings/types";

// -----------------------------------------------------------------------------
// Manager => "update-settings" => Canvas
// Canvas => "update-settings" => Manager
// -----------------------------------------------------------------------------

interface UpdateSettingsPayload {
  id: string;
  settings: Partial<WidgetSettings>;
}

export const updateSettings = {
  on: (handler: EventCallback<UpdateSettingsPayload>) =>
    listen("update-settings", handler),
  toCanvas: (payload: UpdateSettingsPayload) =>
    emitTo("canvas", "update-settings", payload),
  toManager: (payload: UpdateSettingsPayload) =>
    emitTo("manager", "update-settings", payload),
};
