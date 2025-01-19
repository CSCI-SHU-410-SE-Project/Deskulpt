/**
 * This file contains wrappers around the Tauri event system.
 *
 * This helps to prevent typos in event targets and names, and provides type checking
 * for the event payloads.
 */

import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import {
  RemoveWidgetsPayload,
  RenderWidgetPayload,
  UpdateSettingsPayload,
} from "./types/frontend";
import { Appearance, ShowToastPayload } from "./types/backend";

/**
 * Emit the "render-widget" event to the canvas window.
 *
 * @param payload The payload of the event.
 */
export async function emitRenderWidgetToCanvas(payload: RenderWidgetPayload) {
  await emitTo("canvas", "render-widget", payload);
}

/**
 * Listen to the "render-widget" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToRenderWidget(
  handler: EventCallback<RenderWidgetPayload>,
) {
  return listen("render-widget", handler);
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
 * Emit the "switch-theme-appearance" event to the canvas window.
 *
 * @param payload The payload of the event.
 */
export async function emitSwitchAppearanceToCanvas(payload: Appearance) {
  await emitTo("canvas", "switch-theme-appearance", payload);
}

/**
 * Listen to the "switch-theme-appearance" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export function listenToSwitchAppearance(handler: EventCallback<Appearance>) {
  return listen("switch-theme-appearance", handler);
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
