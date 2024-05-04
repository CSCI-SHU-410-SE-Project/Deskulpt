/**
 * This file contains wrappers around the Tauri command system.
 *
 * This helps to prevent typos in command names, provides type checking for the invoke
 * arguments, and ensures correct type hint for the output.
 */

import { invoke } from "@tauri-apps/api/core";
import { Settings, WidgetConfigCollection } from "./types";

export async function invokeBundleWidget(widgetId: string, apisBlobUrl: string) {
  return invoke<string>("bundle_widget", { widgetId, apisBlobUrl });
}

export async function invokeExitApp(settings: Settings) {
  return invoke<null>("exit_app", { settings });
}

export async function invokeOpenWidgetDirectory(widgetId: string | null) {
  return invoke<null>("open_widget_directory", { widgetId });
}

export async function invokeInitSettings() {
  return invoke<Settings>("init_settings");
}

export async function invokeRefreshWidgetCollection() {
  return invoke<WidgetConfigCollection>("refresh_widget_collection");
}

export async function invokeRegisterToggleShortcut(shortcut: string, reverse: boolean) {
  return invoke<null>("register_toggle_shortcut", { shortcut, reverse });
}
