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

export async function emitRenderWidgetToCanvas(payload: RenderWidgetPayload) {
  await emitTo("canvas", "render-widget", payload);
}

export async function listenToRenderWidget(
  handler: EventCallback<RenderWidgetPayload>,
) {
  return listen("render-widget", handler);
}

export async function emitRemoveWidgetsToCanvas(payload: RemoveWidgetsPayload) {
  await emitTo("canvas", "remove-widgets", payload);
}

export async function listenToRemoveWidgets(
  handler: EventCallback<RemoveWidgetsPayload>,
) {
  return listen("remove-widgets", handler);
}

export async function emitUpdateSettingToManager(payload: UpdateSettingPayload) {
  await emitTo("manager", "update-setting", payload);
}

export async function listenToUpdateSetting(
  handler: EventCallback<UpdateSettingPayload>,
) {
  return listen("update-setting", handler);
}

export async function emitSwitchThemeAppearanceToCanvas(payload: ThemeAppearance) {
  await emitTo("canvas", "switch-theme-appearance", payload);
}

export async function listenToSwitchThemeAppearance(
  handler: EventCallback<ThemeAppearance>,
) {
  return listen("switch-theme-appearance", handler);
}

// The "exit-app" event is emitted by the backend to the manager

export async function listenToExitApp(handler: EventCallback<null>) {
  return listen("exit-app", handler);
}

// The "show-toast" event is emitted by the backend to the canvas, though possibly in
// the future it might be emitted by the manager or the canvas as well

export async function listenToShowToast(handler: EventCallback<ShowToastPayload>) {
  return listen("show-toast", handler);
}
