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
    const unlisten = events.renderWidgetsEvent.listen(async (event) => {
      const state = useWidgetsStore.getState();

      const promises = Object.entries(event.payload).map(async ([id, code]) => {
        let apisBlobUrl;
        if (id in state) {
          // APIs blob URL can be reused because the contents are dependent only
          // on widget ID; the code blob URL will definitely change on re-render
          // so we revoke it here
          const widget = state[id]!; // We've checked id in state
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
          );
          return;
        }

        updateWidgetRender(id, module.default, moduleBlobUrl, apisBlobUrl);
      });

      await Promise.all(promises);
    });

    if (!hasInited.current) {
      commands
        .windowReady()
        .then(() => {
          hasInited.current = true;
        })
        .catch(console.error);
    }

    return () => {
      hasInited.current = false;
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
