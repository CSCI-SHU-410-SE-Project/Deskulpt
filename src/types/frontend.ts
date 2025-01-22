/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import { FC } from "react";
import { Result, WidgetConfig, WidgetSettings } from "./backend";

/**
 * The user-defined widget interface.
 *
 * The entry file of each user-defined widget should export an object that fulfills this
 * interface as default.
 */
export interface Widget {
  Component: FC<{ id: string }>;
  /** Widget widget as accepted in CSS. */
  width?: string;
  /** Widget height as accepted in CSS. */
  height?: string;
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
  id: string;
  /** The widget-specific settings to update. */
  settings: WidgetSettings;
}
