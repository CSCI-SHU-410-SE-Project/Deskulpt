import { useEffect, useRef } from "react";
import {
  updateWidgetRender,
  updateWidgetRenderError,
  useWidgetsStore,
} from "./useWidgetsStore";
import { stringifyError } from "../../utils/stringifyError";
import { commands, events } from "../../bindings";

const BASE_URL = new URL(import.meta.url).origin;
const RAW_APIS_URL = new URL("/gen/raw-apis.js", BASE_URL).href;

export function useRenderWidgetsListener() {
  const hasInited = useRef(false);

  useEffect(() => {
    const unlisten = events.renderWidgets.listen(async (event) => {
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
            code = await commands.core.bundleWidget({
              id,
              baseUrl: BASE_URL,
              apisBlobUrl,
            });
          } catch (error) {
            updateWidgetRenderError(
              id,
              "Error bundling the widget",
              stringifyError(error),
              apisBlobUrl,
              settings,
            );
            return;
          }
        }

        const moduleBlob = new Blob([code], { type: "application/javascript" });
        const moduleBlobUrl = URL.createObjectURL(moduleBlob);
        let module;
        try {
          module = await import(/* @vite-ignore */ moduleBlobUrl);
          if (module.default === undefined) {
            throw new Error("Missing default export");
          }
        } catch (error) {
          URL.revokeObjectURL(moduleBlobUrl);
          updateWidgetRenderError(
            id,
            "Error importing the widget module",
            stringifyError(error),
            apisBlobUrl,
            settings,
          );
          return;
        }

        updateWidgetRender(
          id,
          module.default,
          moduleBlobUrl,
          apisBlobUrl,
          settings,
        );
      });

      await Promise.all(promises);
    });

    if (!hasInited.current) {
      // Set the canvas as ready to render only once
      commands.core
        .setRenderReady()
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
