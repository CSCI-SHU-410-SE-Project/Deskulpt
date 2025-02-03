/**
 * This file contains wrappers around the Tauri event system.
 *
 * This helps to prevent typos in event targets and names, and provides type checking
 * for the event payloads.
 */

import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import {
  RemoveWidgetsPayload,
  RenderWidgetsPayload,
  UpdateSettingsPayload,
} from "./types/frontend";
import { ShowToastPayload } from "./types/backend";

/**
 * Emit the "render-widgets" event to the canvas window.
 *
 * @param payload The payload of the event.
 */
export async function emitRenderWidgetsToCanvas(payload: RenderWidgetsPayload) {
  await emitTo("canvas", "render-widgets", payload);
}

/**
 * Listen to the "render-widgets" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToRenderWidgets(
  handler: EventCallback<RenderWidgetsPayload>,
) {
  return listen("render-widgets", handler);
}

/**
 * Emit the "remove-widgets" event to the canvas window.
 *
 * @param payload The payload of the event.
 */
export async function emitRemoveWidgetsToCanvas(payload: RemoveWidgetsPayload) {
  await emitTo("canvas", "remove-widgets", payload);
}

/**
 * Listen to the "remove-widgets" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToRemoveWidgets(
  handler: EventCallback<RemoveWidgetsPayload>,
) {
  return listen("remove-widgets", handler);
}

/**
 * Emit the "update-settings" event to the manager window.
 *
 * @param payload The payload of the event.
 */
export async function emitUpdateSettingsToManager(
  payload: UpdateSettingsPayload,
) {
  await emitTo("manager", "update-settings", payload);
}

/**
 * Listen to the "update-settings" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToUpdateSettings(
  handler: EventCallback<UpdateSettingsPayload>,
) {
  return listen("update-settings", handler);
}

/**
 * Emit the "toggle-theme" event to the canvas window.
 */
export async function emitToggleThemeToCanvas() {
  await emitTo("canvas", "toggle-theme");
}

/**
 * Listen to the "toggle-theme" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToToggleTheme(handler: EventCallback<void>) {
  return listen("toggle-theme", handler);
}

/**
 * Listen to the "exit-app" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToExitApp(handler: EventCallback<null>) {
  return listen("exit-app", handler);
}

/**
 * Listen to the "show-toast" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToShowToast(handler: EventCallback<ShowToastPayload>) {
  return listen("show-toast", handler);
}
