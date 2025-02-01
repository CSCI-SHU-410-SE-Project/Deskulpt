/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import { ReactNode } from "react";
import { Result, WidgetConfig, WidgetSettings } from "./backend";

export type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P];
};

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
  /** Settings of the widget. */
  settings: WidgetSettings;
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
  /** Settings of the widget. */
  settings: WidgetSettings;
  /** The URL of the blob of widget APIs. */
  apisBlobUrl: string;
  /** The URL of the blob of the widget module. */
  moduleBlobUrl?: string;
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
  /** The widget-specific settings. */
  settings: WidgetSettings;
}

/**
 * The payload of the "remove-widgets" event.
 */
export interface RemoveWidgetsPayload {
  /** The widget IDs to remove. */
  removedIds: string[];
}

/**
 * The payload of the "update-settings" event.
 */
export interface UpdateSettingsPayload {
  /** The widget ID. */
  widgetId: string;
  /** The widget-specific settings to update. */
  settings: WidgetSettings;
}
