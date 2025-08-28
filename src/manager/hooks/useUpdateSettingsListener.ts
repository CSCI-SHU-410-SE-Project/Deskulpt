import { useEffect } from "react";
import { updateWidgetSettings } from "./useWidgetsStore";
import { UpdateSettingsEventAPI } from "../../bindings/events";

export function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = UpdateSettingsEventAPI.listen((event) => {
      const { id, ...settings } = event.payload;
      updateWidgetSettings(id, settings);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
