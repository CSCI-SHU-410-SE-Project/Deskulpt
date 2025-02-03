import {
  Dispatch,
  SetStateAction,
  useCallback,
  useEffect,
  useState,
} from "react";
import { ManagerWidgetState } from "../../types/frontend";
import { WidgetSettings } from "../../types/backend";
import { invokeEmitOnRenderReady, invokeRescanWidgets } from "../../commands";
import {
  emitRemoveWidgetsToCanvas,
  emitRenderWidgetsToCanvas,
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
  const rescanAndRenderHelper = async (
    initial: boolean,
    managerWidgetStates: Record<string, ManagerWidgetState>,
    setManagerWidgetStates: Dispatch<
      SetStateAction<Record<string, ManagerWidgetState>>
    >,
  ) => {
    const detectedConfigs = await invokeRescanWidgets();

    // If a widget exists in the previous states but does not exist in the newdetected
    // configurations, we consider it as removed from the collection
    const ids = Object.keys(managerWidgetStates).filter(
      (id) => !(id in detectedConfigs),
    );
    if (ids.length > 0) {
      // Notify the cacnvas to clean up resources allocated for removed widgets
      await emitRemoveWidgetsToCanvas({ ids });
    }

    const newManagerWidgetStates = Object.fromEntries(
      Object.entries(detectedConfigs).map(([id, config]) => {
        let settings: WidgetSettings;
        if (id in managerWidgetStates) {
          // The widget state already exists, from which we can get its settings
          settings = managerWidgetStates[id].settings;
        } else if (
          id in window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgets
        ) {
          // There is an initial setting for the widget
          settings =
            window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgets[id];
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

    const payload = addedStates.map(([id, { settings }]) => ({ id, settings }));
    if (initial) {
      await invokeEmitOnRenderReady({ payload });
    } else {
      await emitRenderWidgetsToCanvas(payload);
    }

    return addedStates.length;
  };

  const rescanAndRender = useCallback(
    () =>
      rescanAndRenderHelper(false, managerWidgetStates, setManagerWidgetStates),
    [managerWidgetStates, setManagerWidgetStates],
  );

  useEffect(() => {
    rescanAndRenderHelper(true, {}, setManagerWidgetStates).catch(
      console.error,
    );
  }, [setManagerWidgetStates]);

  return { managerWidgetStates, setManagerWidgetStates, rescanAndRender };
}
