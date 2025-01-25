import { invoke } from "@tauri-apps/api/core";
import { Settings, Shortcuts, WidgetConfig } from "../types";

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

export function invokeWindowReady(payload: { window: "manager" | "canvas" }) {
  return invoke<void>("window_ready", payload);
}
