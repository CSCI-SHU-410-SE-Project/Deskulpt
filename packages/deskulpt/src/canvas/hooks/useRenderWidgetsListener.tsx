import { useEffect, useRef } from "react";
import { useWidgetsStore } from "./useWidgetsStore";
import { commands, events } from "../../bindings";

export function useRenderWidgetsListener() {
  const hasInited = useRef(false);

  useEffect(() => {
    const unlisten = events.renderWidgets.listen(async (event) => {
      // Just ensure widgets exist in store - individual webviews handle their own rendering
      const widgets = useWidgetsStore.getState();

      event.payload.forEach((id) => {
        if (!(id in widgets)) {
          // Create placeholder widget state for webview management
          useWidgetsStore.setState((state) => ({
            ...state,
            [id]: {
              component: () => null, // Placeholder, will be replaced by individual webview
              apisBlobUrl: "", // Will be created by individual webview
            },
          }));
        }
      });
    });

    if (!hasInited.current) {
      // Set the canvas as ready to render only once
      commands.core
        .setRenderReady()
        .then(() => {
          hasInited.current = true;
        })
        .catch(console.error);
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
