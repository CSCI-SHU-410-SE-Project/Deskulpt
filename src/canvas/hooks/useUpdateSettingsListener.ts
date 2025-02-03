import { useEffect } from "react";
import { listenToUpdateSettings } from "../../events";
import { updateWidgetSettings } from "./useWidgetsStore";

export function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = listenToUpdateSettings((event) => {
      const { id, settings } = event.payload;
      updateWidgetSettings(id, settings);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
