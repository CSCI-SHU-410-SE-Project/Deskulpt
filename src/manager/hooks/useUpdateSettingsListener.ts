import { useEffect } from "react";
import { events } from "../../core";
import { updateWidgetSettings } from "./useWidgetsStore";

export default function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = events.updateSettings.on((event) => {
      const { id, settings } = event.payload;
      updateWidgetSettings(id, settings);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
