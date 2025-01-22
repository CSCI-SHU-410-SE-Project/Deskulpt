/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import { Result, WidgetConfig, WidgetSettings } from "./backend";

/**
 * The state of a widget in the manager.
 */
export interface ManagerWidgetState {
  /** Configuration or configuration error of the widget. */
  config: Result<WidgetConfig, string>;
  /** Settings of the widget. */
  settings: WidgetSettings;
}
