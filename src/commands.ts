/**
 * This file contains wrappers around the Tauri command system.
 *
 * This helps to prevent typos in command names, provides type checking for the invoke
 * arguments, and ensures correct type hint for the output.
 */

import { invoke } from "@tauri-apps/api/core";
import { Settings } from "./types";

export async function invokeBundleWidget(widgetId: string, apisBlobUrl: string) {
  return invoke<string>("bundle_widget", { widgetId, apisBlobUrl });
}

export async function invokeExitApp(settings: Settings) {
  return invoke<null>("exit_app", { settings });
}
