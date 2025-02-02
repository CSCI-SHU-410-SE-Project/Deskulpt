/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import { WidgetConfig, WidgetSettings } from "./backend";

export type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P];
};

/**
 * The state of a widget in the manager.
 */
export interface ManagerWidgetState {
  /** Configuration of the widget. */
  config: WidgetConfig;
  /** Settings of the widget. */
  settings: WidgetSettings;
}

/**
 * The payload of the "render-widget" event.
 */
export interface RenderWidgetPayload {
  /** The widget ID. */
  id: string;
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
  id: string;
  /** The widget-specific settings to update. */
  settings: WidgetSettings;
}
