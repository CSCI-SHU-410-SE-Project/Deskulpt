/**
 * This file contains the types and interfaces that are used purely in the frontend,
 * without corresponding backend implementations.
 */

import ReactDOM from "react-dom/client";
import { Widget } from ".";
import { WidgetConfig } from "./backend";

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
  config: WidgetConfig;
  /** [FRONTEND] Import URL of the widget APIs. */
  apisBlobUrl: string;
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
 * The module obtained by dynamically importing the bundle of widget source code.
 */
export interface WidgetModule {
  /** The default export of the entry file of a user-defined widget. */
  default: Widget;
}

/**
 * The HTML and React DOM roots for rendering a widget.
 */
export interface WidgetDOMRoot {
  html: HTMLDivElement;
  react: ReactDOM.Root;
}

/**
 * The record of a widget on the canvas.
 */
export interface WidgetRecord {
  /**
   * The HTML and React DOM roots in which the widget is rendered.
   *
   * To completely remove a widget from the canvas, one need to call the `unmount`
   * method on `domRoot.react` and the `remove` method on `domRoot.html`, if possible.
   */
  root: WidgetDOMRoot;
  /**
   * Whether the widget is being rendered.
   *
   * If the widget is not being rendered, the corresponding error should be rendered in
   * the DOM root instead.
   */
  error: boolean;
}
