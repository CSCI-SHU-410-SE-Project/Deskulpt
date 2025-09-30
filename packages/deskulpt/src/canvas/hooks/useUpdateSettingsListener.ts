import { useEffect } from "react";
import { updateWidgetSettings } from "./useWidgetsStore";
import { events } from "../../bindings";

export function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = events.updateSettingsEvent.listen((event) => {
      const { id, ...settings } = event.payload;
      updateWidgetSettings(id, settings);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
