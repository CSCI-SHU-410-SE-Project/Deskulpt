import { useCallback, useEffect } from "react";
import { listenToRenderWidget } from "../../events";
import { WidgetSettings } from "../../types/backend";
import { invokeBundleWidget } from "../../commands";
import {
  Widget,
  updateWidgetRender,
  updateWidgetRenderError,
  updateWidgetSettings,
  useWidgetsStore,
} from "./useWidgetsStore";

// The base URL used for resolving local path imports
const baseUrl = new URL(import.meta.url).origin;

/**
 * Listen and react to the "render-widget" event.
 */
export default function useRenderWidgetListener() {
  const bundleWidget = useCallback(
    async (id: string, settings: WidgetSettings) => {
      // Get the widget APIs blob URL, reusing if applicable
      let apisBlobUrl: string;
      const widgets = useWidgetsStore.getState().widgets;
      if (id in widgets) {
        apisBlobUrl = widgets[id].apisBlobUrl;
      } else {
        const apisCode = window.__DESKULPT_CANVAS_INTERNALS__.apisWrapper
          .replace("__DESKULPT_WIDGET_ID__", id)
          .replace(
            "__RAW_APIS_URL__",
            new URL("/generated/raw-apis.js", baseUrl).href,
          );
        const apisBlob = new Blob([apisCode], {
          type: "application/javascript",
        });
        apisBlobUrl = URL.createObjectURL(apisBlob);
      }

      // Bundle the widget and get the output code
      let moduleCode: string;
      try {
        moduleCode = await invokeBundleWidget({
          id,
          baseUrl,
          apisBlobUrl,
        });
      } catch (err) {
        updateWidgetRenderError(id, err, apisBlobUrl, settings);
        return;
      }

      // Create the module blob URL
      const moduleBlob = new Blob([moduleCode], {
        type: "application/javascript",
      });
      const moduleBlobUrl = URL.createObjectURL(moduleBlob);

      // Dynamically import the module and render the widget
      try {
        const module = await import(/* @vite-ignore */ moduleBlobUrl);
        const widget = module.default as Widget;
        if (widget === undefined || widget.Component === undefined) {
          updateWidgetRenderError(
            id,
            "The widget must provide a default export with a `Component` property.",
            apisBlobUrl,
            settings,
          );
          return;
        }

        // We have validated the module so we can call `render` safely; there could be
        // error within `render` so it needs to called in advance, otherwise things will
        // get broken in the state setter, causing the error to be uncaught and also
        // affecting other widget displays
        updateWidgetRender(id, widget, moduleBlobUrl, apisBlobUrl, settings);
      } catch (err) {
        updateWidgetRenderError(id, err, apisBlobUrl, settings);
      }
    },
    [],
  );

  useEffect(() => {
    const unlisten = listenToRenderWidget((event) => {
      const { id, bundle, settings } = event.payload;

      // We do not wish to bundle the widget
      if (!bundle) {
        // Make sure that we do not update settings of a not-yet-rendered widget; note
        // that is not an errorneous case because users can update settings in the
        // manager without having rendered them on the canvas; the case is, when the
        // manager finally requests to bundle, it will carry the latest settings in the
        // payload so the canvas can still get the correct information
        updateWidgetSettings(id, settings);
        return;
      }

      // We do wish to bundle the widget
      bundleWidget(id, settings).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [bundleWidget]);
}
