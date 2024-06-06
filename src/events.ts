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
  UpdateSettingPayload,
} from "./types/frontend";
import { ShowToastPayload, ThemeAppearance } from "./types/backend";

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
export async function listenToRenderWidget(
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
export async function listenToRemoveWidgets(
  handler: EventCallback<RemoveWidgetsPayload>,
) {
  return listen("remove-widgets", handler);
}

/**
 * Emit the "update-setting" event to the manager window.
 *
 * @param payload The payload of the event.
 */
export async function emitUpdateSettingToManager(payload: UpdateSettingPayload) {
  await emitTo("manager", "update-setting", payload);
}

/**
 * Listen to the "update-setting" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export async function listenToUpdateSetting(
  handler: EventCallback<UpdateSettingPayload>,
) {
  return listen("update-setting", handler);
}

/**
 * Emit the "switch-theme-appearance" event to the canvas window.
 *
 * @param payload The payload of the event.
 */
export async function emitSwitchThemeAppearanceToCanvas(payload: ThemeAppearance) {
  await emitTo("canvas", "switch-theme-appearance", payload);
}

/**
 * Listen to the "switch-theme-appearance" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export async function listenToSwitchThemeAppearance(
  handler: EventCallback<ThemeAppearance>,
) {
  return listen("switch-theme-appearance", handler);
}

/**
 * Listen to the "exit-app" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export async function listenToExitApp(handler: EventCallback<null>) {
  return listen("exit-app", handler);
}

/**
 * Listen to the "show-toast" event.
 *
 * @param handler The callback function to handle the event.
 * @returns A promise that resolves to a function to unlisten to the event.
 */
export async function listenToShowToast(handler: EventCallback<ShowToastPayload>) {
  return listen("show-toast", handler);
}
