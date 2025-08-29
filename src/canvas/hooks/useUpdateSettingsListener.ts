import { useEffect } from "react";
import { updateWidgetSettings } from "./useWidgetsStore";
import { UpdateSettingsEventAPI } from "../../bindings/events";

export function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = UpdateSettingsEventAPI.listen((event) => {
      const { widgets } = event.payload;
      updateWidgetSettings(widgets);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
