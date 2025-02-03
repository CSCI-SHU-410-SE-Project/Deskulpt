import { useEffect } from "react";
import { listenToBatchRemove } from "../../core/events";
import { removeWidgets, useWidgetsStore } from "./useWidgetsStore";

export function useBatchRemoveListener() {
  useEffect(() => {
    const unlisten = listenToBatchRemove((event) => {
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
