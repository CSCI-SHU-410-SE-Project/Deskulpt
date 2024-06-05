/**
 * This file contains wrappers around the Tauri command system.
 *
 * This helps to prevent typos in command names, provides type checking for the invoke
 * arguments, and ensures correct type hint for the output.
 */

import { invoke } from "@tauri-apps/api/core";
import { Settings, WidgetConfigCollection } from "./types/backend";

export async function invokeBundleWidget(widgetId: string, apisBlobUrl: string) {
  return invoke<string>("bundle_widget", { widgetId, apisBlobUrl });
}

export async function invokeExitApp(settings: Settings) {
  return invoke<void>("exit_app", { settings });
}

export async function invokeOpenWidgetResource(
  widgetId: string | null,
  path: string | null,
) {
  return invoke<void>("open_widget_resource", { widgetId, path });
}

export async function invokeInitSettings() {
  return invoke<Settings>("init_settings");
}

export async function invokeRefreshWidgetCollection() {
  return invoke<WidgetConfigCollection>("refresh_widget_collection");
}

export async function invokeRegisterToggleShortcut(shortcut: string, reverse: boolean) {
  return invoke<void>("register_toggle_shortcut", { shortcut, reverse });
}
