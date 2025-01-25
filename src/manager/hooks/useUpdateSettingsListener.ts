import { useEffect, useRef } from "react";
import { listenToUpdateSettings } from "../../core/events";
import { WidgetsActionType, WidgetsDispatch } from "./useWidgets";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";

export function useUpdateSettingsListener(
  widgetsDispatch: WidgetsDispatch,
  ready: ReadyCallback,
) {
  const isReady = useRef(false);

  useEffect(() => {
    const unlisten = listenToUpdateSettings((event) => {
      const { id, settings } = event.payload;
      widgetsDispatch({
        type: WidgetsActionType.SET_SETTINGS,
        payload: { id, settings },
      });
    });

    if (!isReady.current) {
      ready(ListenerKeys.UPDATE_SETTINGS);
      isReady.current = true;
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [widgetsDispatch, ready]);
}
