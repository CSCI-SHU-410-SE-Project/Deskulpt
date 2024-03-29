import React from "react";

import { Event as TauriEvent, listen } from "@tauri-apps/api/event";
import { renderWidget } from "./render";
import { RenderWidgetEventPayload } from "../types";

window.__DESKULPT__ = {
  React, // Allows user-defined widgets to use the same React instance as Deskulpt
  widgetStore: {},
};

const canvas = document.getElementById("canvas")!;

// Listen to the "render-widget" event, emitted by the manager
// Note: we avoid using top-level await since it is a relatively new ECMAScript feature
(async () => {
  await listen("render-widget", (event: TauriEvent<RenderWidgetEventPayload>) => {
    renderWidget(canvas, event.payload);
  });
})();
