import { useEffect } from "react";
import { listenToRemoveWidgets } from "../../events";
import { removeWidgets, useWidgetsStore } from "./useWidgetsStore";

export function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = listenToRemoveWidgets((event) => {
      const { ids } = event.payload;
      const widgets = useWidgetsStore.getState().widgets;

      ids.forEach((id) => {
        const widget = widgets[id];
        if (widget === null) {
          return; // This should not happen but let us be safe
        }
        URL.revokeObjectURL(widget.apisBlobUrl);
        if (widget.moduleBlobUrl !== undefined) {
          URL.revokeObjectURL(widget.moduleBlobUrl);
        }
      });

      removeWidgets(ids);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
