import { useCallback } from "react";
import { BASE_URL } from "../consts";
import { WidgetSettings } from "../../types/backend";
import { invokeBundleWidget } from "../../core/commands";
import { Widget } from "../../types/frontend";
import { WidgetsActionType, WidgetsDispatch, WidgetsState } from "./useWidgets";

export type RenderCallback = (
  id: string,
  settings?: WidgetSettings,
  code?: string,
) => void;

/**
 * Return a callback function for rendering a widget.
 *
 * @param widgets The widgets state.
 * @param widgetsDispatch The widgets dispatch function.
 * @returns The callback function `render`.
 *
 * ### `render`
 *
 * This function will add a new widget if the ID is not in the widgets state, or
 * update the properties of an existing widget otherwise.
 *
 * @param id The widget ID.
 * @param settings The widget settings. Required and used only when the widget
 *  is not in the the widgets state.
 * @param code The widget code. If provided, the bundling step will be skipped.
 */
export function useRenderCallback(
  widgets: WidgetsState,
  widgetsDispatch: WidgetsDispatch,
) {
  const render = useCallback<RenderCallback>(
    async (id: string, settings?: WidgetSettings, code?: string) => {
      let apisBlobUrl;

      if (id in widgets) {
        // The APIs blob URL can be reused because the contents are dependent only
        // on widget ID
        const widget = widgets[id];
        apisBlobUrl = widgets[id].apisBlobUrl;
        if (widget.moduleBlobUrl !== undefined) {
          URL.revokeObjectURL(widget.moduleBlobUrl);
        }
      } else {
        // Create new APIs blob URL from the template
        const apisCode = window.__DESKULPT_INTERNALS__.apisTemplate
          .replace("__DESKULPT_WIDGET_ID__", id)
          .replace(
            "__RAW_APIS_URL__",
            new URL("/.scripts/raw-apis.js", BASE_URL).href,
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
    },
    [widgets],
  );

  return render;
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
