import { invoke } from "@tauri-apps/api/core";
import { Settings, Shortcuts, WidgetConfig } from "../types";
import { RenderPayload } from "./events";

export function invokeBundleWidget(payload: {
  id: string;
  baseUrl: string;
  apisBlobUrl: string;
}) {
  return invoke<string>("bundle_widget", payload);
}

export function invokeExitApp(payload: { settings: Settings }) {
  return invoke<void>("exit_app", payload);
}

export function invokeOpenInWidgetsDir(payload: { components: string[] }) {
  return invoke<void>("open_in_widgets_dir", payload);
}

export function invokeRescanWidgets() {
  return invoke<{
    configMap: Record<string, WidgetConfig>;
    addedIds: string[];
    removedIds: string[];
  }>("rescan_widgets");
}

export function invokeUpdateShortcuts(payload: {
  oldShortcuts: Shortcuts;
  newShortcuts: Shortcuts;
}) {
  return invoke<void>("update_shortcuts", payload);
}

export function invokeEmitOnRenderReady(payload: { payload: RenderPayload }) {
  return invoke<void>("emit_on_render_ready", payload);
}

export function invokeSetRenderReady() {
  return invoke<void>("set_render_ready");
}
