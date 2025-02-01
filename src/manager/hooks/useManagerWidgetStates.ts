import {
  Dispatch,
  SetStateAction,
  useCallback,
  useEffect,
  useState,
} from "react";
import { ManagerWidgetState } from "../../types/frontend";
import { WidgetSettings } from "../../types/backend";
import { invokeRescanWidgets } from "../../commands";
import {
  emitRemoveWidgetsToCanvas,
  emitRenderWidgetToCanvas,
} from "../../events";

export interface UseManagerWidgetStatesOutput {
  /** The manager widget states. */
  managerWidgetStates: Record<string, ManagerWidgetState>;
  /** Setter for the manager widget states. */
  setManagerWidgetStates: Dispatch<
    SetStateAction<Record<string, ManagerWidgetState>>
  >;
  /** Function that scans the widget base directory and renders newly added widgets. */
  rescanAndRender: () => Promise<number>;
}

/**
 * Hook for initializing the manager widget states.
 *
 * This initializes the manager widget states with the initial widget settings,
 * and prepares the setter and the `rescanAndRender` function that is the core
 * function for refreshing the widget configuration map. This will also perform
 * an initial scanning and render on mount with a small timeout.
 */
export default function useManagerWidgetStates(): UseManagerWidgetStatesOutput {
  const [managerWidgetStates, setManagerWidgetStates] = useState<
    Record<string, ManagerWidgetState>
  >({});

  /**
   * Rescan the widget base directory and render newly added widgets.
   *
   * Newly added widgets are those that exist in the new states but does not exist in
   * the previous states. They will be rendered and the number of newly added widgets
   * will be returned.
   */
  const rescanAndRender = useCallback(async () => {
    const detectedConfigs = await invokeRescanWidgets();

    // If a widget exists in the previous states but does not exist in the newdetected
    // configurations, we consider it as removed from the collection
    const removedIds = Object.keys(managerWidgetStates).filter(
      (id) => !(id in detectedConfigs),
    );
    if (removedIds.length > 0) {
      // Notify the cacnvas to clean up resources allocated for removed widgets
      await emitRemoveWidgetsToCanvas({ removedIds });
    }

    const newManagerWidgetStates = Object.fromEntries(
      Object.entries(detectedConfigs).map(([id, config]) => {
        let settings: WidgetSettings;
        if (id in managerWidgetStates) {
          // The widget state already exists, from which we can get its settings
          settings = managerWidgetStates[id].settings;
        } else if (
          id in
          window.__DESKULPT_MANAGER_INTERNALS__.initialSettings
            .widgetSettingsMap
        ) {
          // There is an initial setting for the widget
          settings =
            window.__DESKULPT_MANAGER_INTERNALS__.initialSettings
              .widgetSettingsMap[id];
        } else {
          // Fall back to the default setting
          settings = { x: 0, y: 0, opacity: 100 };
        }
        return [id, { config, settings }];
      }),
    );

    const addedStates = Object.entries(newManagerWidgetStates).filter(
      ([id]) => !(id in managerWidgetStates),
    );
    setManagerWidgetStates(newManagerWidgetStates); // Direct replacement
    await Promise.all(
      addedStates.map(([id, { settings }]) =>
        emitRenderWidgetToCanvas({ id, settings, bundle: true }),
      ),
    );
    return addedStates.length;
  }, [managerWidgetStates, setManagerWidgetStates]);

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
