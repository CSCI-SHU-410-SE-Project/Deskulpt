import { useEffect, useRef } from "react";
import { BASE_URL } from "../consts";
import { invokeBundleWidget } from "../../core/commands";
import {
  Widget,
  WidgetsActionType,
  WidgetsDispatch,
  WidgetsState,
} from "./useWidgets";
import { listenToRender } from "../../core/events";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";

export function useRenderListener(
  widgets: WidgetsState,
  widgetsDispatch: WidgetsDispatch,
  ready: ReadyCallback,
) {
  const isReady = useRef(false);

  useEffect(() => {
    const unlisten = listenToRender(async (event) => {
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
          const apisCode = window.__DESKULPT__.apisWrapper
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

          if (id in widgets) {
            widgetsDispatch({
              type: WidgetsActionType.SET_RENDER,
              payload: { id, widget, moduleBlobUrl },
            });
          } else if (settings !== undefined) {
            widgetsDispatch({
              type: WidgetsActionType.ADD,
              payload: { id, widget, settings, apisBlobUrl, moduleBlobUrl },
            });
          }
        } catch (error) {
          if (id in widgets) {
            widgetsDispatch({
              type: WidgetsActionType.SET_RENDER_ERROR,
              payload: { id, error },
            });
          } else if (settings !== undefined) {
            widgetsDispatch({
              type: WidgetsActionType.ADD_ERROR,
              payload: { id, error, settings, apisBlobUrl },
            });
          }
        }
      });

      await Promise.all(promises);
    });

    if (!isReady.current) {
      ready(ListenerKeys.RENDER);
      isReady.current = true;
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [widgets]);
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
