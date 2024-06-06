/**
 * This file contains wrappers around the Tauri command system.
 *
 * This helps to prevent typos in command names, provides type checking for the invoke
 * arguments, and ensures correct type hint for the output.
 */

import { invoke } from "@tauri-apps/api/core";
import { Settings, WidgetConfigCollection } from "./types/backend";

/**
 * Invoke the [`bundle_widget`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.bundle_widget.html) command.
 *
 * @param widgetId The ID of the widget to bundle.
 * @param apisBlobUrl The URL of the APIs blob associated with the widget.
 * @returns The bundled widget code as a string.
 */
export async function invokeBundleWidget(widgetId: string, apisBlobUrl: string) {
  return invoke<string>("bundle_widget", { widgetId, apisBlobUrl });
}

/**
 * Invoke the [`exit_app`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.exit_app.html) command.
 *
 * @param settings The settings to save before exiting the app.
 */
export async function invokeExitApp(settings: Settings) {
  return invoke<void>("exit_app", { settings });
}

/**
 * Invoke the [`open_widget_resource`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.open_widget_resource.html) command.
 * @param widgetId The ID of the widget to open resource for, or `null` to open the
 * widget base directory.
 * @param path The relative path of the resource to open with respect to the widget
 * directory, or `null` to stand for `"."`.
 */
export async function invokeOpenWidgetResource(
  widgetId: string | null,
  path: string | null,
) {
  return invoke<void>("open_widget_resource", { widgetId, path });
}

/**
 * Invoke the [`init_settings`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.init_settings.html) command.
 *
 * @returns The initial settings read from the previously saved settings file.
 */
export async function invokeInitSettings() {
  return invoke<Settings>("init_settings");
}

/**
 * Invoke the [`refresh_widget_collection`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.refresh_widget_collection.html) command.
 *
 * @returns The refreshed widget configuration collection.
 */
export async function invokeRefreshWidgetCollection() {
  return invoke<WidgetConfigCollection>("refresh_widget_collection");
}

/**
 * Invoke the [`register_toggle_shortcut`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.register_toggle_shortcut.html) command.
 *
 * @param shortcut The shortcut to register/unregister.
 * @param reverse Whether to register (`false`) or unregister (`true`).
 * @returns
 */
export async function invokeRegisterToggleShortcut(shortcut: string, reverse: boolean) {
  return invoke<void>("register_toggle_shortcut", { shortcut, reverse });
}
