import { invoke } from "@tauri-apps/api/core";
import { Settings, SettingsUpdate, WidgetConfig } from "../types";
import { RenderWidgetsPayload } from "./events";

export const bundleWidget = (payload: {
  id: string;
  baseUrl: string;
  apisBlobUrl: string;
}) => invoke<string>("bundle_widget", payload);

export const emitOnRenderReady = (payload: { payload: RenderWidgetsPayload }) =>
  invoke<void>("emit_on_render_ready", payload);

export const exitApp = (payload: { settings: Settings }) =>
  invoke<void>("exit_app", payload);

export const openWidget = (payload?: { id?: string }) =>
  invoke<void>("open_widget", payload);

export const rescanWidgets = () =>
  invoke<Record<string, WidgetConfig>>("rescan_widgets");

export const setRenderReady = () => invoke<void>("set_render_ready");

export const updateSettings = (payload: { updates: SettingsUpdate[] }) =>
  invoke<void>("update_settings", payload);
