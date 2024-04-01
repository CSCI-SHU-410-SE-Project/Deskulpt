import React from "react";
import ReactDOM from "react-dom/client";

import { Event as TauriEvent, listen } from "@tauri-apps/api/event";
import {
  RenderWidgetPayload,
  WidgetDOMRoot,
  WidgetModule,
  WidgetRecord,
} from "../types";
import { grabErrorInfo } from "../utils";

window.__DESKULPT__ = { defaultDeps: { React } };

const canvas = document.getElementById("canvas")!;
const widgetRecords: Record<string, WidgetRecord> = {};

// Listen to the "render-widget" event, emitted by the manager
listen("render-widget", (event: TauriEvent<RenderWidgetPayload>) => {
  const { widgetId, bundlerOutput } = event.payload;

  if ("success" in bundlerOutput) {
    // In this case the bundler output wraps the bundled code; we create an object URL
    // so as to dynamically import the bundled code and obtain its export
    const blob = new Blob([bundlerOutput.success], {
      type: "application/javascript",
    });
    const url = URL.createObjectURL(blob);

    import(/* @vite-ignore */ url)
      .then((module: WidgetModule) => {
        // Re-use an existing DOM root with the same ID or create a new one; the reason
        // we do it early here is that even when rendering fails (expected or not), we
        // would need a DOM root to render error messages nicely (except in rare cases
        // that we are unable to render even just for errors); this also means that we
        // have to carefully handle all possible cases in the proceeding steps to ensure
        // that there are no "dead" memory consumers left on the canvas
        let widgetDOMRoot: WidgetDOMRoot | null = null;
        try {
          if (widgetId in widgetRecords) {
            // Re-use the existing DOM root
            widgetDOMRoot = widgetRecords[widgetId].root;
          } else {
            // Create a new DOM root and append to the canvas
            const htmlDOMRoot = document.createElement("div");
            htmlDOMRoot.id = `deskulpt-widget--${widgetId}`;
            const reactDOMRoot = ReactDOM.createRoot(htmlDOMRoot);
            canvas.appendChild(htmlDOMRoot);
            widgetDOMRoot = { html: htmlDOMRoot, react: reactDOMRoot };
          }
        } catch (err) {
          // The most likely reason for the error is that the react DOM root cannot be
          // successfully created; in that case we must avoid leaving an unused HTML div on
          // the canvas if it has already been created
          console.error(err); // @Charlie-XIAO better error handling
          const unusedDiv = document.getElementById(`deskulpt-widget--${widgetId}`);
          if (unusedDiv) {
            unusedDiv.remove();
          }
          return;
        }

        const widget = module.default;
        try {
          // Render widget element or error message
          try {
            widgetDOMRoot.react.render(
              <React.StrictMode>{widget.render()}</React.StrictMode>,
            );
          } catch (err) {
            widgetDOMRoot.react.render(
              <React.StrictMode>
                <div>{grabErrorInfo(err)}</div>
              </React.StrictMode>,
            );
          }
        } catch (err) {
          // Unmount and remove DOM root since we cannot recover from this case; this
          // can happen if rendering even fails for the error message (e.g., there are
          // issues with the DOM root)
          console.error(err); // @Charlie-XIAO better error handling
          widgetDOMRoot.react.unmount();
          widgetDOMRoot.html.remove();
        }

        // The rendering is successful if we reach here, so we specify no error
        widgetRecords[widgetId] = { root: widgetDOMRoot, widget };
      })
      .catch((err: Error) => {
        // FIXME: handle the error
        console.log({ err });
      });
  } else {
    // FIXME: handle the unsuccessful case
    console.log({ bundlerOutput });
  }
})
  .then((unlisten) => {
    window.addEventListener("beforeunload", unlisten);
  })
  .catch((err) => {
    console.error(err);
  });
