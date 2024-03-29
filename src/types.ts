/**
 * The collection of types and interfaces for the Deskulpt frontend.
 */

import React from "react";
import ReactDOM from "react-dom/client";

// --- Global interfaces ---------------------------------------------------------------

declare global {
  interface Window {
    __DESKULPT__: {
      React: typeof React;
      widgetStore: Record<string, WidgetDetails>;
    };
  }
}

// --- User interfaces -----------------------------------------------------------------

export interface Widget {
  init: () => void;
  render: () => React.FC;
  destroy: () => void;
}

// --- Backend structs -----------------------------------------------------------------

export interface WidgetConfig {
  deskulpt_conf: DeskulptConf;
  package_json: PackageJson;
  directory: string;
}

export interface DeskulptConf {
  name: string;
  entry: string;
  ignore: boolean;
}

export interface PackageJson {
  dependencies: Record<string, string>;
}

// --- Backend commands ----------------------------------------------------------------

export interface BundlerOutputPayload {
  success: boolean;
  message: string;
}

// --- Event payloads ------------------------------------------------------------------

export interface RenderWidgetEventPayload {
  widgetId: string;
  bundlerOutputPayload: BundlerOutputPayload;
}

// --- Frontend structs ----------------------------------------------------------------

export interface WidgetModule {
  default: Widget;
}

export interface WidgetDetails {
  widget: Widget | null;
  domRoot: ReactDOM.Root;
}
