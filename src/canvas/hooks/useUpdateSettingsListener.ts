import { useEffect } from "react";
import { listenToUpdateSettings } from "../../core/events";
import { WidgetsActionType, WidgetsDispatch } from "./useWidgets";

export function useUpdateSettingsListener(widgetsDispatch: WidgetsDispatch) {
  useEffect(() => {
    const unlisten = listenToUpdateSettings((event) => {
      const { id, settings } = event.payload;
      widgetsDispatch({
        type: WidgetsActionType.SET_SETTINGS,
        payload: { id, settings },
      });
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
