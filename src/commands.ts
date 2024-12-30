/**
 * This file contains wrappers around the Tauri command system.
 *
 * This helps to prevent typos in command names, provides type checking for the invoke
 * arguments, and ensures correct type hint for the output.
 */

import { invoke } from "@tauri-apps/api/core";
import { Settings, WidgetConfigMap } from "./types/backend";

/**
 * Invoke the `bundle_widget` command.
 */
export async function invokeBundleWidget(widgetId: string, apisBlobUrl: string) {
  return invoke<string>("bundle_widget", { widgetId, apisBlobUrl });
}

/**
 * Invoke the `exit_app` command.
 */
export async function invokeExitApp(settings: Settings) {
  return invoke<void>("exit_app", { settings });
}

/**
 * Invoke the `open_widget_resource` command.
 */
export async function invokeOpenWidgetResource(
  widgetId: string | null,
  path: string | null,
) {
  return invoke<void>("open_widget_resource", { widgetId, path });
}

/**
 * Invoke the `init_settings` command.
 */
export async function invokeInitSettings() {
  return invoke<Settings>("init_settings");
}

/**
 * Invoke the `refresh_widget_collection` command.
 */
export async function invokeRefreshWidgetCollection() {
  return invoke<WidgetConfigMap>("refresh_widget_collection");
}

/**
 * Invoke the `register_toggle_shortcut` command.
 */
export async function invokeRegisterToggleShortcut(shortcut: string, reverse: boolean) {
  return invoke<void>("register_toggle_shortcut", { shortcut, reverse });
}
