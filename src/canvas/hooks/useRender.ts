import { useEffect, useRef, useState } from "react";
import { BASE_URL } from "../consts";
import { invokeBundleWidget, invokeSetRenderReady } from "../../core/commands";

import { listenToRender } from "../../core/events";
import {
  Widget,
  updateWidgetRender,
  updateWidgetRenderError,
  useWidgetsStore,
} from "./useWidgetsStore";

export function useRender() {
  const [isRendering, setIsRendering] = useState(true);
  const hasInited = useRef(false);

  useEffect(() => {
    const unlisten = listenToRender(async (event) => {
      // If rendering is done within 1s, we do not set loading state at all
      const timer = setTimeout(() => setIsRendering(true), 1000);

      const widgets = useWidgetsStore.getState().widgets;
      const promises = event.payload.map(async ({ id, settings, code }) => {
        let apisBlobUrl;

        if (id in widgets) {
          // The APIs blob URL can be reused because the contents are dependent
          // only on widget ID
          const widget = widgets[id];
          apisBlobUrl = widgets[id].apisBlobUrl;
          if (widget.moduleBlobUrl !== undefined) {
            URL.revokeObjectURL(widget.moduleBlobUrl);
          }
        } else {
          // Create new APIs blob URL from the template
          const apisCode = window.__DESKULPT_CANVAS_INTERNALS__.apisWrapper
            .replace("__DESKULPT_WIDGET_ID__", id)
            .replace(
              "__RAW_APIS_URL__",
              new URL("/generated/raw-apis.js", BASE_URL).href,
            );
          const apisBlob = new Blob([apisCode], {
            type: "application/javascript",
          });
          apisBlobUrl = URL.createObjectURL(apisBlob);
        }

        try {
          const { widget, moduleBlobUrl } = await renderHelper(
            id,
            apisBlobUrl,
            code,
          );
          updateWidgetRender(id, widget, moduleBlobUrl, apisBlobUrl, settings);
        } catch (error) {
          updateWidgetRenderError(id, error, apisBlobUrl, settings);
        }
      });

      await Promise.all(promises);
      clearTimeout(timer);
      setIsRendering(false);
    });

    if (!hasInited.current) {
      invokeSetRenderReady()
        .then(() => {
          hasInited.current = true;
        })
        .catch(console.error);
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [setIsRendering]);

  return isRendering;
}

async function renderHelper(id: string, apisBlobUrl: string, code?: string) {
  // Bundle the widget if code is not provided
  if (code === undefined) {
    code = await invokeBundleWidget({ id, baseUrl: BASE_URL, apisBlobUrl });
  }

  // Import the widget code
  const blob = new Blob([code], { type: "application/javascript" });
  const moduleBlobUrl = URL.createObjectURL(blob);
  let module;
  try {
    module = await import(/* @vite-ignore */ moduleBlobUrl);
  } catch (err) {
    URL.revokeObjectURL(moduleBlobUrl);
    throw err;
  }

  const widget = module.default as Widget;
  if (widget === undefined || widget.Component === undefined) {
    URL.revokeObjectURL(moduleBlobUrl);
    throw new Error(
      "The widget must provide a default export with a `Component` property.",
    );
  }
  return { widget, moduleBlobUrl };
}
