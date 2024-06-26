import { Dispatch, SetStateAction, useEffect } from "react";
import { listenToRemoveWidgets } from "../../events";
import { CanvasWidgetState } from "../../types/frontend";
import { IdMap } from "../../types/backend";

/**
 * Listen and react to the "remove-widgets" event.
 *
 * @param canvasWidgetStates Canvas widget states.
 * @param setCanvasWidgetStates Setter for the canvas widget states.
 */
export default function useRemoveWidgetsListener(
  canvasWidgetStates: IdMap<CanvasWidgetState>,
  setCanvasWidgetStates: Dispatch<SetStateAction<IdMap<CanvasWidgetState>>>,
) {
  useEffect(() => {
    const unlisten = listenToRemoveWidgets((event) => {
      const { removedIds } = event.payload;

      removedIds.forEach((widgetId) => {
        const state = canvasWidgetStates[widgetId];
        if (state == null) {
          return; // This should not happen but let us be safe
        }

        // Revoke the blob URLs because they will not be automatically cleaned up, and
        // being in the removed IDs means that they will be removed from the canvas
        // states and the next time they show up, they will be assigned new blob URLs
        URL.revokeObjectURL(state.apisBlobUrl);
        if (state.moduleBlobUrl) {
          URL.revokeObjectURL(state.moduleBlobUrl);
        }
      });

      // Remove the specified widgets from the canvas states
      setCanvasWidgetStates((prev) =>
        Object.fromEntries(
          Object.entries(prev).filter(([widgetId]) => !removedIds.includes(widgetId)),
        ),
      );
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [canvasWidgetStates]);
}
