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
 */
export async function invokeBundleWidget(widgetId: string, apisBlobUrl: string) {
  return invoke<string>("bundle_widget", { widgetId, apisBlobUrl });
}

/**
 * Invoke the [`bundle_external_dependencies`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.bundle_external_dependencies.html) command.
 */
export async function invokeBundleExternalDependencies(widgetId: string) {
  return invoke<string>("bundle_external_dependencies", { widgetId });
}

/**
 * Invoke the [`exit_app`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.exit_app.html) command.
 */
export async function invokeExitApp(settings: Settings) {
  return invoke<void>("exit_app", { settings });
}

/**
 * Invoke the [`open_widget_resource`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.open_widget_resource.html) command.
 */
export async function invokeOpenWidgetResource(
  widgetId: string | null,
  path: string | null,
) {
  return invoke<void>("open_widget_resource", { widgetId, path });
}

/**
 * Invoke the [`init_settings`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.init_settings.html) command.
 */
export async function invokeInitSettings() {
  return invoke<Settings>("init_settings");
}

/**
 * Invoke the [`refresh_widget_collection`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.refresh_widget_collection.html) command.
 */
export async function invokeRefreshWidgetCollection() {
  return invoke<WidgetConfigCollection>("refresh_widget_collection");
}

/**
 * Invoke the [`register_toggle_shortcut`](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/commands/fn.register_toggle_shortcut.html) command.
 */
export async function invokeRegisterToggleShortcut(shortcut: string, reverse: boolean) {
  return invoke<void>("register_toggle_shortcut", { shortcut, reverse });
}
