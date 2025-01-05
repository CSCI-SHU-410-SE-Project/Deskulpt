import { Dispatch, SetStateAction, useEffect } from "react";
import { listenToRenderWidget } from "../../events";
import { CanvasWidgetState, WidgetModule } from "../../types/frontend";
import { WidgetSettings } from "../../types/backend";
import { invokeBundleWidget } from "../../commands";
import ErrorDisplay from "../components/ErrorDisplay";
import { grabErrorInfo } from "../utils";

// The default width and height of a widget container, used when the widget module
// fails to be loaded correctly
const defaultContainerWidth = "300px";
const defaultContainerHeight = "150px";

/**
 * Listen and react to the "render-widget" event.
 *
 * @param canvasWidgetStates Canvas widget states.
 * @param setCanvasWidgetStates Setter for the canvas widget states.
 */
export default function useRenderWidgetListener(
  canvasWidgetStates: Record<string, CanvasWidgetState>,
  setCanvasWidgetStates: Dispatch<SetStateAction<Record<string, CanvasWidgetState>>>,
) {
  useEffect(() => {
    const unlisten = listenToRenderWidget((event) => {
      const { widgetId, bundle, settings } = event.payload;
      const isTracked = widgetId in canvasWidgetStates;

      // We do not wish to bundle the widget
      if (!bundle) {
        // Make sure that we do not update settings of a not-yet-rendered widget; note
        // that is not an errorneous case because users can update settings in the
        // manager without having rendered them on the canvas; the case is, when the
        // manager finally requests to bundle, it will carry the latest settings in the
        // payload so the canvas can still get the correct information
        if (isTracked) {
          setCanvasWidgetStates((prev) => ({
            ...prev,
            [widgetId]: { ...prev[widgetId], settings },
          }));
        }
        return;
      }

      // We do wish to bundle the widget
      bundleWidget(widgetId, settings, isTracked).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [canvasWidgetStates]);

  /**
   * Bundle the widget and update the canvas state.
   *
   * This function assigns or reuses the APIs blob URL, invokes the backend to bundle
   * the widget, dynamically import the bundled code and update the canvas state (if
   * possible) so that it can be displayed on the canvas.
   */
  async function bundleWidget(
    widgetId: string,
    settings: WidgetSettings,
    isTracked: boolean,
  ) {
    // Get the widget APIs blob URL, reusing if applicable
    let apisBlobUrl: string;
    if (isTracked) {
      apisBlobUrl = canvasWidgetStates[widgetId].apisBlobUrl;
    } else {
      const apisCode = await getWidgetApisCode(widgetId);
      const apisBlob = new Blob([apisCode], { type: "application/javascript" });
      apisBlobUrl = URL.createObjectURL(apisBlob);
    }

    // Bundle the widget and get the output code
    let moduleCode: string;
    try {
      moduleCode = await invokeBundleWidget({ widgetId, apisBlobUrl });
    } catch (err) {
      setCanvasWidgetStates((prev) => ({
        ...prev,
        [widgetId]: {
          display: (
            <ErrorDisplay
              title={`Error in '${widgetId}': failed to bundle widget`}
              error={grabErrorInfo(err)}
            />
          ),
          width: defaultContainerWidth,
          height: defaultContainerHeight,
          settings,
          apisBlobUrl,
        },
      }));
      return;
    }

    // Create the module blob URL
    const moduleBlob = new Blob([moduleCode], { type: "application/javascript" });
    const moduleBlobUrl = URL.createObjectURL(moduleBlob);

    // Dynamically import the module and render the widget
    try {
      const module = (await import(/* @vite-ignore */ moduleBlobUrl)) as WidgetModule;
      const moduleError = getWidgetModuleError(module);

      if (moduleError !== null) {
        // There are known errors with the widget module
        setCanvasWidgetStates((prev) => ({
          ...prev,
          [widgetId]: {
            display: (
              <ErrorDisplay
                title={`Error in '${widgetId}': invalid widget module`}
                error={moduleError}
              />
            ),
            width: defaultContainerWidth,
            height: defaultContainerHeight,
            moduleBlobUrl,
            settings,
            apisBlobUrl,
          },
        }));
        return;
      }

      // We have validated the module so we can call `render` safely; there could be
      // error within `render` so it needs to called in advance, otherwise things will
      // get broken in the state setter, causing the error to be uncaught and also
      // affecting other widget displays
      const widgetDisplay = module.default.render();
      setCanvasWidgetStates((prev) => ({
        ...prev,
        [widgetId]: {
          display: widgetDisplay,
          width: module.default.width,
          height: module.default.height,
          moduleBlobUrl,
          settings,
          apisBlobUrl,
        },
      }));
    } catch (err) {
      setCanvasWidgetStates((prev) => ({
        ...prev,
        [widgetId]: {
          display: (
            <ErrorDisplay
              title={`Error in '${widgetId}': failed to import widget module`}
              error={grabErrorInfo(err)}
            />
          ),
          width: defaultContainerWidth,
          height: defaultContainerHeight,
          moduleBlobUrl,
          settings,
          apisBlobUrl,
        },
      }));
    }
  }
}

/**
 * Get the code for the widget-specific APIs.
 *
 * This function fetches the template of widget APIs and replaces the placeholder with
 * the actual widget ID.
 */
async function getWidgetApisCode(widgetId: string) {
  // The template is in the public directory, bundled from `packages/apis`
  const response = await fetch("/.wrap-apis.js.txt");
  const template = await response.text();
  return template.replace("__DESKULPT_WIDGET_ID__", widgetId);
}

/**
 * Validate a user widget module.
 *
 * If the module is invalid, the function returns an error message, and otherwise it
 * returns `null`. It ensures that the module provides a default export that contains
 * a `render` function.
 */
function getWidgetModuleError(module: WidgetModule) {
  const widget = module.default;
  if (widget === undefined) {
    return "The widget must provide a default export.";
  }
  if (widget.render === undefined) {
    return "The default export of the widget must provide a `render` function.";
  }
  if (typeof widget.render !== "function") {
    return "The `render` key of the default export must be a function.";
  }
  if (widget.width === undefined || widget.height === undefined) {
    return "The widget must provide `width` and `height` properties.";
  }
  return null;
}
