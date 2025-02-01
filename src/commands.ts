/**
 * This file contains wrappers around the Tauri command system.
 *
 * This helps to prevent typos in command names, provides type checking for the invoke
 * arguments, and ensures correct type hint for the output.
 */

import { invoke } from "@tauri-apps/api/core";
import { Settings, Shortcuts, WidgetConfig } from "./types/backend";

/**
 * Invoke the `bundle_widget` command.
 */
export function invokeBundleWidget(payload: {
  id: string;
  baseUrl: string;
  apisBlobUrl: string;
}) {
  return invoke<string>("bundle_widget", payload);
}

/**
 * Invoke the `exit_app` command.
 */
export function invokeExitApp(payload: { settings: Settings }) {
  return invoke<void>("exit_app", payload);
}

/**
 * Invoke the `open_in_widgets_dir` command.
 */
export function invokeOpenInWidgetsDir(payload: { components: string[] }) {
  return invoke<void>("open_in_widgets_dir", payload);
}

/**
 * Invoke the `rescan_widgets` command.
 */
export function invokeRescanWidgets() {
  return invoke<Record<string, WidgetConfig>>("rescan_widgets");
}

/**
 * Invoke the `update_shortcut` command.
 */
export function invokeUpdateShortcut(payload: {
  key: keyof Shortcuts;
  oldShortcut: string | null;
  newShortcut: string | null;
}) {
  return invoke<void>("update_shortcut", payload);
}
