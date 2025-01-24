import { useEffect } from "react";
import { WidgetSettings } from "../../types";
import { invokeRescanWidgets } from "../../core/commands";
import { emitBatchRemoveToCanvas, emitRenderToCanvas } from "../../core/events";
import { WidgetsActionType, useWidgets } from "./useWidgets";

export function useManagerWidgetStates() {
  const [widgets, widgetsDispatch] = useWidgets();

  /**
   * Get the new widget states in the manager.
   *
   * This function fetches the widget collection by scanning the widget base directory.
   * If notifies the canvas about removed widget, and returns the updated widget states
   * as detected configurations, wrapped with either existing or initial settings.
   */
  async function getNewManagerWidgetStates() {
    const { configMap, removedIds } = await invokeRescanWidgets();

    if (removedIds.length > 0) {
      // Notify the cacnvas to clean up resources allocated for removed widgets
      await emitBatchRemoveToCanvas({ ids: removedIds });
    }

    // Return the new states, wrapped from the detected configurations; note that we are
    // only caring about the detected configurations
    return Object.fromEntries(
      Object.entries(configMap).map(([id, config]) => {
        let settings: WidgetSettings;
        if (id in widgets) {
          settings = widgets[id].settings;
        } else if (
          id in window.__DESKULPT__.initialSettings.widgetSettingsMap
        ) {
          settings = window.__DESKULPT__.initialSettings.widgetSettingsMap[id];
        } else {
          settings = { x: 0, y: 0, opacity: 100 };
        }
        return [id, { config, settings }];
      }),
    );
  }

  /**
   * Rescan the widget base directory and render newly added widgets.
   *
   * Newly added widgets are those that exist in the new states but does not exist in
   * the previous states. They will be rendered and the number of newly added widgets
   * will be returned.
   */
  async function rescanAndRender() {
    const newManagerWidgetStates = await getNewManagerWidgetStates();
    const addedStates = Object.entries(newManagerWidgetStates).filter(
      ([id]) => !(id in widgets),
    );
    widgetsDispatch({
      type: WidgetsActionType.BATCH_UPDATE,
      payload: { widgets: newManagerWidgetStates },
    });
    await Promise.all(
      addedStates.map(([id, { settings }]) =>
        emitRenderToCanvas({ id, settings }),
      ),
    );
    return addedStates.length;
  }

  useEffect(() => {
    // The rescan is guaranteed to succeed because it triggers a command in the backend;
    // the rendering, however, mail fail due to the canvas not being ready to receive
    // the rendering events; this should be rare with a 1.5-second timeout
    const timer = setTimeout(() => {
      rescanAndRender().catch(console.error);
    }, 1500);

    return () => {
      clearTimeout(timer);
    };
  }, []);

  return rescanAndRender;
}
