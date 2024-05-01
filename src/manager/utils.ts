import { invoke } from "@tauri-apps/api/core";
import { ManagerWidgetState, WidgetConfigCollection, WidgetSetting } from "../types";
import { emitRemoveWidgetsToCanvas, emitRenderWidgetToCanvas } from "../events";

/**
 * Get the new widget states in the manager.
 *
 * This function fetches the widget collection by scanning the widget base directory.
 * If notifies the canvas about removed widget, and returns the updated widget states
 * as detected configurations, wrapped with either existing or initial settings.
 */
export async function getNewManagerWidgetStates(
  managerWidgetStates: Record<string, ManagerWidgetState>,
  initialWidgetSettings: Record<string, WidgetSetting>,
): Promise<Record<string, ManagerWidgetState>> {
  const detectedConfigs = await invoke<WidgetConfigCollection>(
    "refresh_widget_collection",
  );

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
        setting = { x: 0, y: 0 };
      }
      return [widgetId, { config, setting }];
    }),
  );
}

/**
 * Notify the canvas to render a collection of widgets.
 *
 * The event is emitted for each widget asynchronously in parallel.
 */
export async function renderWidgets(
  managerWidgetStates: Record<string, ManagerWidgetState>,
) {
  await Promise.all(
    Object.entries(managerWidgetStates).map(([widgetId, { setting }]) =>
      emitRenderWidgetToCanvas({ widgetId, setting, bundle: true }),
    ),
  );
}
