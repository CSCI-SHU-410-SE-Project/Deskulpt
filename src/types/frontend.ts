/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import { ReactNode } from "react";
import { Widget } from ".";
import { Result, WidgetConfig, WidgetInternal } from "./backend";

/**
 * The state of a widget.
 *
 * We cannot manage widget states purely in the backend or in the frontend. Managing
 * purely in the backend may cause too frequent communication when the frontend needs
 * information. Managing purely in the frontend would cause use to send large objects
 * to the backend when sometimes we can just send the widget ID.
 *
 * The frontend widget state thus consists of (1) a shared part that is synced with the
 * backend at times, and (2) a frontend part that does not bother the backend.
 */
export interface WidgetState {
  /** [SHARED] Widget configuration. */
  config: Result<WidgetConfig, string>;
  /** [FRONTEND] Import URL of the widget APIs. */
  apisBlobUrl: string;
}

/**
 * The state of a widget on the canvas.
 */
export interface WidgetCanvasState {
  /** The internals of the widget. */
  internal: WidgetInternal;
  /** The rendered widget component or the error component to display. */
  display: ReactNode;
}

/**
 * The payload of the "render-widget" event.
 */
export interface RenderWidgetPayload {
  widgetId: string;
  success: boolean;
  /** The bundled code if `success` is `true` or the bundler error. */
  bundlerOutput: string;
}

/**
 * The payload of the "remove-widgets" event.
 */
export interface RemoveWidgetsPayload {
  removedIds: string[];
}

/**
 * The module obtained by dynamically importing the bundle of widget source code.
 */
export interface WidgetModule {
  /** The default export of the entry file of a user-defined widget. */
  default: Widget;
}
