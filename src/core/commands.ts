import { invoke } from "@tauri-apps/api/core";
import { Settings, WidgetCollection } from "../types/backend";

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

export function invokeLoadSettings() {
  return invoke<Settings>("load_settings");
}

export function invokeRescanWidgets() {
  return invoke<WidgetCollection>("rescan_widgets");
}

export function invokeUpdateToggleShortcut(payload: {
  oldShortcut?: string;
  newShortcut?: string;
}) {
  return invoke<void>("update_toggle_shortcut", payload);
}
