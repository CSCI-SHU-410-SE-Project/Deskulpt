import { useEffect, useRef } from "react";
import { listenToRenderWidgets } from "../../events";
import { invokeBundleWidget, invokeSetRenderReady } from "../../commands";
import {
  Widget,
  updateWidgetRender,
  updateWidgetRenderError,
  useWidgetsStore,
} from "./useWidgetsStore";

const BASE_URL = new URL(import.meta.url).origin;
const RAW_APIS_URL = new URL("/generated/raw-apis.js", BASE_URL).href;

export function useRenderWidgetsListener() {
  const hasInited = useRef(false);

  useEffect(() => {
    const unlisten = listenToRenderWidgets(async (event) => {
      const widgets = useWidgetsStore.getState().widgets;

      const promises = event.payload.map(async ({ id, settings, code }) => {
        let apisBlobUrl;
        if (id in widgets) {
          // APIs blob URL can be reused because the contents are dependent only
          // on widget ID; the code blob URL will definitely change on re-render
          // so we revoke it here
          const widget = widgets[id];
          apisBlobUrl = widget.apisBlobUrl;
          if (widget.moduleBlobUrl !== undefined) {
            URL.revokeObjectURL(widget.moduleBlobUrl);
          }
        } else {
          const apisCode = window.__DESKULPT_CANVAS_INTERNALS__.apisWrapper
            .replace("__DESKULPT_WIDGET_ID__", id)
            .replace("__RAW_APIS_URL__", RAW_APIS_URL);
          const apisBlob = new Blob([apisCode], {
            type: "application/javascript",
          });
          apisBlobUrl = URL.createObjectURL(apisBlob);
        }

        if (code === undefined) {
          // If code is not provided, we need to bundle the widget
          try {
            code = await invokeBundleWidget({
              id,
              baseUrl: BASE_URL,
              apisBlobUrl,
            });
          } catch (error) {
            updateWidgetRenderError(id, error, apisBlobUrl, settings);
            return;
          }
        }

        const moduleBlob = new Blob([code], { type: "application/javascript" });
        const moduleBlobUrl = URL.createObjectURL(moduleBlob);
        let module;
        try {
          module = await import(/* @vite-ignore */ moduleBlobUrl);
        } catch (error) {
          URL.revokeObjectURL(moduleBlobUrl);
          updateWidgetRenderError(id, error, apisBlobUrl, settings);
          return;
        }

        const widget = module.default as Widget;
        if (widget === undefined || widget.Component === undefined) {
          URL.revokeObjectURL(moduleBlobUrl);
          updateWidgetRenderError(
            id,
            "The widget must provide a default export with a `Component` property.",
            apisBlobUrl,
            settings,
          );
          return;
        }

        updateWidgetRender(id, widget, moduleBlobUrl, apisBlobUrl, settings);
      });

      await Promise.all(promises);
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
  }, []);
}
