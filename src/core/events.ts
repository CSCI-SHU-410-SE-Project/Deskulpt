import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import { Theme, WidgetSettings } from "../bindings/types";

// -----------------------------------------------------------------------------
// Manager => "remove-widgets" => Canvas
// -----------------------------------------------------------------------------

interface RemoveWidgetsPayload {
  ids: string[];
}

export const removeWidgets = {
  on: (handler: EventCallback<RemoveWidgetsPayload>) =>
    listen("remove-widgets", handler),
  toCanvas: (payload: RemoveWidgetsPayload) =>
    emitTo("canvas", "remove-widgets", payload),
};

// -----------------------------------------------------------------------------
// Manager => "switch-theme" => Canvas
// -----------------------------------------------------------------------------

interface SwitchThemePayload {
  theme: Theme;
}

export const switchTheme = {
  on: (handler: EventCallback<SwitchThemePayload>) =>
    listen("switch-theme", handler),
  toCanvas: (payload: SwitchThemePayload) =>
    emitTo("canvas", "switch-theme", payload),
};

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
