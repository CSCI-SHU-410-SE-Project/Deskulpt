import { useEffect } from "react";
import { events } from "../../bindings";
import { useWidgetsStore } from "./useWidgetsStore";

export function useUpdateWidgetCatalogListener() {
  useEffect(() => {
    const unlisten = events.updateWidgetCatalog.listen((event) => {
      // Clean up widgets that are no longer in the catalog
      useWidgetsStore.setState((state) => {
        const newState = { ...state };
        Object.entries(state).forEach(
          ([id, { apisBlobUrl, moduleBlobUrl }]) => {
            if (id in event.payload) {
              return;
            }
            URL.revokeObjectURL(apisBlobUrl);
            if (moduleBlobUrl !== undefined) {
              URL.revokeObjectURL(moduleBlobUrl);
            }
            delete newState[id];
          },
        );
        return newState;
      }, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
