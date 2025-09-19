import { useEffect } from "react";
import { events } from "../../bindings";
import { useSettings, useWidgetConfigRegistry } from "./useStores";

export function useEventListeners() {
  useUpdateSettingsListener();
  useUpdateWidgetConfigRegistryListener();
}

function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = events.updateSettingsEvent.listen((event) => {
      useSettings.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}

function useUpdateWidgetConfigRegistryListener() {
  useEffect(() => {
    const unlisten = events.updateWidgetConfigRegistryEvent.listen((event) => {
      useWidgetConfigRegistry.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
