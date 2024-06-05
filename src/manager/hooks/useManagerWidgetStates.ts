import { useEffect, useState } from "react";
import { ManagerWidgetState } from "../../types/frontend";
import { IdMap, WidgetSetting } from "../../types/backend";
import { invokeRefreshWidgetCollection } from "../../commands";
import { emitRemoveWidgetsToCanvas } from "../../events";
import { renderWidgets } from "../utils";

export default function useManagerWidgetStates(
  initialWidgetSettings: IdMap<WidgetSetting>,
) {
  const [managerWidgetStates, setManagerWidgetStates] = useState<
    IdMap<ManagerWidgetState>
  >({});

  /**
   * Get the new widget states in the manager.
   *
   * This function fetches the widget collection by scanning the widget base directory.
   * If notifies the canvas about removed widget, and returns the updated widget states
   * as detected configurations, wrapped with either existing or initial settings.
   */
  async function getNewManagerWidgetStates() {
    const detectedConfigs = await invokeRefreshWidgetCollection();

    // If a widget exists in the previous states but does not exist in the new detected
    // configurations, we consider it as removed from the collection
    const removedIds = Object.keys(managerWidgetStates).filter(
      (id) => !(id in detectedConfigs),
    );
    if (removedIds.length > 0) {
      // Notify the cacnvas to clean up resources allocated for removed widgets
      await emitRemoveWidgetsToCanvas({ removedIds });
    }

    // Return the new states, wrapped from the detected configurations; note that we are
    // only caring about the detected configurations
    return Object.fromEntries(
      Object.entries(detectedConfigs).map(([widgetId, config]) => {
        let setting: WidgetSetting;
        if (widgetId in managerWidgetStates) {
          // The widget state already exists, from which we can get its settings
          setting = managerWidgetStates[widgetId].setting;
        } else if (widgetId in initialWidgetSettings) {
          // There is an initial setting for the widget
          setting = initialWidgetSettings[widgetId];
        } else {
          // Fall back to the default setting
          setting = { x: 0, y: 0, opacity: 100 };
        }
        return [widgetId, { config, setting }];
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
    const addedStatesRecords = Object.entries(newManagerWidgetStates).filter(
      ([widgetId]) => !(widgetId in managerWidgetStates),
    );
    const addedStates = Object.fromEntries(addedStatesRecords);
    setManagerWidgetStates(newManagerWidgetStates); // Direct replacement
    await renderWidgets(addedStates);
    return addedStatesRecords.length;
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

  return { managerWidgetStates, setManagerWidgetStates, rescanAndRender };
}
