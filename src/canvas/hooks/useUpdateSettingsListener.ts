import { useEffect } from "react";
import { events } from "../../bindings";
import { useSettingsStore } from "./useSettingsStore";

export function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = events.updateSettingsEvent.listen((event) => {
      useSettingsStore.setState(() => ({ settings: event.payload }), true);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
