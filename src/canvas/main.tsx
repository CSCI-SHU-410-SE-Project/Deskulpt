import React from "react";

import { Event as TauriEvent, listen } from "@tauri-apps/api/event";
import { RenderWidgetPayload, WidgetModule, WidgetRecord } from "../types";
import { grabErrorInfo, handleError, getDOMRoot } from "./utils";
import WidgetContainer from "../components/WidgetContainer";

// // explicitly import the default dependencies so that it won't be tree-shaken
// import "../../default_deps_dist/react"
// import "../../default_deps_dist/apis";

// These three lines will add `@deskulpt/react` and `@deskulpt/apis` to the import graph.
// Without these lines, the bundler will tree-shake the two modules and widget api will not be bundled
import * as reactDummy from "@deskulpt/react";
import * as apisDummy from "@deskulpt/apis";
console.log(reactDummy, apisDummy);

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
        const widgetDOMRoot = getDOMRoot(widgetId, widgetRecords, canvas);
        if (widgetDOMRoot === null) {
          return;
        }

        // Early return before rendering if there are known errors in the widget
        const widget = module.default;
        if (widget === undefined) {
          handleError(
            widgetId,
            widgetDOMRoot,
            widgetRecords,
            `Widget (id=${widgetId}) is invalid`,
            "The widget entry file does not provide a default export.",
          );
          return;
        }
        if (widget.render === undefined || typeof widget.render !== "function") {
          handleError(
            widgetId,
            widgetDOMRoot,
            widgetRecords,
            `Widget (id=${widgetId}) is invalid`,
            "The object exported by the widget entry file does not have a `render` " +
              "key, or the `render` key does not correspond to a function.",
          );
          return;
        }

        // Try rendering the widget, otherwise render the error information
        try {
          widgetDOMRoot.react.render(
            <WidgetContainer id={widgetId} inner={widget.render()} />,
          );
        } catch (err) {
          handleError(
            widgetId,
            widgetDOMRoot,
            widgetRecords,
            `Widget (id=${widgetId}) fails to be rendered`,
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
            `Widget (id=${widgetId}) fails to be loaded`,
            grabErrorInfo(err),
          );
        }
      });
  } else {
    const widgetDOMRoot = getDOMRoot(widgetId, widgetRecords, canvas);
    if (widgetDOMRoot !== null) {
      handleError(
        widgetId,
        widgetDOMRoot,
        widgetRecords,
        `[Backend] Widget (id=${widgetId}) fails to be bundled`,
        bundlerOutput.failure,
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
