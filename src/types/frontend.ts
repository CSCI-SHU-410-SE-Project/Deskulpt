/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import { ReactNode } from "react";
import { Widget } from ".";
import { Result, WidgetConfig, WidgetSetting } from "./backend";

/**
 * The state of a widget in the manager.
 */
export interface ManagerWidgetState {
  /** Configuration or configuration error of the widget. */
  config: Result<WidgetConfig, string>;
  /** Setting of the widget. */
  setting: WidgetSetting;
}

/**
 * The state of a widget on the canvas.
 */
export interface CanvasWidgetState {
  /** The rendered widget component or the error component to display. */
  display: ReactNode;
  /** The width of the widget container, as exported from the widget module. */
  width: number | string;
  /** The height of the widget container, as exported from the widget module. */
  height: number | string;
  /** Setting of the widget. */
  setting: WidgetSetting;
  /** The URL of the blob of widget APIs. */
  apisBlobUrl: string;
  /** The URL of the blob of the widget module. */
  moduleBlobUrl: string | null;
}

/**
 * The module obtained by dynamically importing the bundle of widget source code.
 */
export interface WidgetModule {
  /** The default export of the entry file of a user-defined widget. */
  default: Widget;
}

/**
 * =====================================================================================
 *
 *    EVENT PAYLOAD TYPES
 *
 * =====================================================================================
 */

export interface RenderWidgetPayload {
  widgetId: string;
  bundle: boolean;
  setting: WidgetSetting;
}

export interface RemoveWidgetsPayload {
  removedIds: string[];
}

export interface UpdateSettingPayload {
  widgetId: string;
  setting: WidgetSetting;
}
