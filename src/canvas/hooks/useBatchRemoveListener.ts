import { useEffect, useRef } from "react";
import { listenToBatchRemove } from "../../core/events";
import { WidgetsActionType, WidgetsDispatch, WidgetsState } from "./useWidgets";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";

export function useBatchRemoveListener(
  widgets: WidgetsState,
  widgetsDispatch: WidgetsDispatch,
  ready: ReadyCallback,
) {
  const isReady = useRef(false);

  useEffect(() => {
    const unlisten = listenToBatchRemove((event) => {
      const { ids } = event.payload;

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

      widgetsDispatch({
        type: WidgetsActionType.BATCH_REMOVE,
        payload: { ids },
      });
    });

    if (!isReady.current) {
      ready(ListenerKeys.BATCH_REMOVE);
      isReady.current = true;
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [widgets, widgetsDispatch, ready]);
}
