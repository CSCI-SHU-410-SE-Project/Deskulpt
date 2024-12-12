/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import { ReactNode } from "react";
import { Result, WidgetConfig, WidgetSetting } from "./backend";

/**
 * The user-defined widget interface.
 *
 * The entry file of each user-defined widget should export an object that fulfills this
 * interface as default.
 */
export interface Widget {
  /** Function that returns the React component to render. */
  render: () => ReactNode;
  /** Widget widget as accepted in CSS. */
  width: string;
  /** Widget height as accepted in CSS. */
  height: string;
}

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
  width: Widget["width"];
  /** The height of the widget container, as exported from the widget module. */
  height: Widget["height"];
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
 * The payload of the "render-widget" event.
 */
export interface RenderWidgetPayload {
  /** The widget ID. */
  widgetId: string;
  /** Whether to call the backend to bundle the widget. */
  bundle: boolean;
  /** The widget-specific setting. */
  setting: WidgetSetting;
}

/**
 * The payload of the "remove-widgets" event.
 */
export interface RemoveWidgetsPayload {
  /** The widget IDs to remove. */
  removedIds: string[];
}

/**
 * The payload of the "update-setting" event.
 */
export interface UpdateSettingPayload {
  /** The widget ID. */
  widgetId: string;
  /** The widget-specific settting to update. */
  setting: WidgetSetting;
}
