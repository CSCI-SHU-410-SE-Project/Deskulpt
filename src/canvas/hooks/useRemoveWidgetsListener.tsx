import { useEffect } from "react";
import { listenToRemoveWidgets } from "../../events";
import { removeWidgets, useWidgetsStore } from "./useWidgetsStore";

/**
 * Listen and react to the "remove-widgets" event.
 */
export default function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = listenToRemoveWidgets((event) => {
      const { removedIds } = event.payload;
      const widgets = useWidgetsStore.getState().widgets;

      removedIds.forEach((id) => {
        const state = widgets[id];
        if (state === null) {
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

      removeWidgets(removedIds);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
