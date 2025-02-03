import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import { Theme, WidgetSettings } from "../types";

// -----------------------------------------------------------------------------
// Backend => "exit-app" => Manager
// -----------------------------------------------------------------------------

export const exitApp = {
  on: (handler: EventCallback<void>) => listen("exit-app", handler),
};

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
// Manager => "render-widgets" => Canvas
// -----------------------------------------------------------------------------

export type RenderWidgetsPayload = {
  id: string;
  settings?: any;
  code?: string;
}[];

export const renderWidgets = {
  on: (handler: EventCallback<RenderWidgetsPayload>) =>
    listen("render-widgets", handler),
  toCanvas: (payload: RenderWidgetsPayload) =>
    emitTo("canvas", "render-widgets", payload),
};

// -----------------------------------------------------------------------------
// Backend => "show-toast" => Manager
// -----------------------------------------------------------------------------

export enum ShowToastPayloadType {
  SUCCESS = "SUCCESS",
  ERROR = "ERROR",
}

type Payload =
  | { type: ShowToastPayloadType.SUCCESS; content: string }
  | { type: ShowToastPayloadType.ERROR; content: string };

export const showToast = {
  on: (handler: EventCallback<Payload>) => listen("show-toast", handler),
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
