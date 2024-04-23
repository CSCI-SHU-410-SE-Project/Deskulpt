import React from "react";
import { Event as TauriEvent, listen } from "@tauri-apps/api/event";
import { RenderWidgetPayload, WidgetModule, WidgetRecord } from "../types";
import { handleError, getDOMRoot, getWidgetModuleError } from "./utils";
import { grabErrorInfo } from "../utils";
import WidgetContainer from "../components/WidgetContainer";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";

import "./main.css";

window.__DESKULPT__ = { defaultDeps: { React } };

// window.addEventListener("click", () => {
//   invoke("set_canvas_to_bottom", {}).catch((err) => {
//     console.error(err);
//   })
// });
// We directly listen to window WM_FOCUS event instead of using the onFocusChanged method
// because the latter is executed only after the window has been focused, which is too late
// await appWindow.onFocusChanged((focused) => {
//   if (focused) {
//     invoke("set_canvas_to_bottom", {}).catch((err) => {
//       console.error(err);
//     });
//   }
// });

// on webpage load, set the canvas to the bottom
document.addEventListener("DOMContentLoaded", () => {
  // Register the onFocusChanged event listener to set the canvas
  // to the bottom whenever the window is focused
  appWindow
    .onFocusChanged((focused) => {
      if (focused.payload) {
        invoke("set_canvas_to_bottom", {}).catch((err) => {
          console.error(err);
        });
      }
    })
    .then(() => {
      // Note that this function must be called after the
      // onFocusChanged event listener is registered. Otherwise
      // no content will be rendered, for unknown reasons.
      invoke("set_canvas_to_bottom", {})
        .catch((err) => {
          console.error(err);
        })
        .catch(console.error);
    })
    .catch(console.error);
});

// // on webpage load, set the canvas to be always at the bottom
// document.addEventListener("DOMContentLoaded", () => {
//   invoke("set_canvas_always_to_bottom", {}).catch((err) => {
//     console.error(err);
//   });
// });

const canvas = document.getElementById("canvas")!;
const widgetRecords: Record<string, WidgetRecord> = {};

// Listen to the "render-widget" event, emitted by the manager
listen("render-widget", (event: TauriEvent<RenderWidgetPayload>) => {
  const { widgetId, success, bundlerOutput } = event.payload;

  if (success) {
    // In this case the bundler output is the bundled code; we create an object URL so
    // that we can dynamically import the bundled code and obtain its export
    const blob = new Blob([bundlerOutput], { type: "application/javascript" });
    const url = URL.createObjectURL(blob);

    import(/* @vite-ignore */ url)
      .then((module: WidgetModule) => {
        const widgetDOMRoot = getDOMRoot(widgetId, widgetRecords, canvas);
        if (widgetDOMRoot === null) {
          return;
        }

        // Early return before rendering if there are known errors in the widget
        const widgetModuleError = getWidgetModuleError(module);
        if (widgetModuleError !== null) {
          handleError(
            widgetId,
            widgetDOMRoot,
            widgetRecords,
            `Error in '${widgetId}': invalid widget module`,
            widgetModuleError,
          );
          return;
        }

        // Try rendering the widget, otherwise render the error information
        const widget = module.default;
        try {
          widgetDOMRoot.react.render(
            <WidgetContainer id={widgetId} inner={widget.render()} />,
          );
        } catch (err) {
          handleError(
            widgetId,
            widgetDOMRoot,
            widgetRecords,
            `Error in '${widgetId}': widget rendering failed (likely a problem with the \`render\` function)`,
            grabErrorInfo(err),
          );
          return;
        }

        // Reaching here means that the widget has been successfully rendered
        widgetRecords[widgetId] = { root: widgetDOMRoot, error: false };
      })
      .catch((err) => {
        const widgetDOMRoot = getDOMRoot(widgetId, widgetRecords, canvas);
        if (widgetDOMRoot !== null) {
          handleError(
            widgetId,
            widgetDOMRoot,
            widgetRecords,
            `Error in '${widgetId}': widget module fails to be imported`,
            grabErrorInfo(err),
          );
        }
      });
  } else {
    // In this case the bundler output is the error message
    const widgetDOMRoot = getDOMRoot(widgetId, widgetRecords, canvas);
    if (widgetDOMRoot !== null) {
      handleError(
        widgetId,
        widgetDOMRoot,
        widgetRecords,
        `[Backend] Widget (id=${widgetId}) fails to be bundled`,
        bundlerOutput,
      );
    }
  }
})
  .then((unlisten) => {
    window.addEventListener("beforeunload", unlisten);
  })
  .catch((err) => {
    console.error(err);
  });
