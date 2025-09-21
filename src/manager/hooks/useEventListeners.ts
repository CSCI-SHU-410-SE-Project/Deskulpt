import { events } from "../../bindings";
import { useSettings, useWidgetConfigRegistry } from "./useStores";
import { useSetupEventListener } from "../../utils/useSetupEventListener";

export function useEventListeners() {
  useUpdateSettingsListener();
  useUpdateWidgetConfigRegistryListener();
}

function useUpdateSettingsListener() {
  useSetupEventListener("managerUpdateSettings", () =>
    events.updateSettingsEvent.listen((event) => {
      useSettings.setState(() => event.payload, true);
    }),
  );
}

function useUpdateWidgetConfigRegistryListener() {
  useSetupEventListener("managerUpdateWidgetConfigRegistry", () =>
    events.updateWidgetConfigRegistryEvent.listen((event) => {
      useWidgetConfigRegistry.setState(() => event.payload, true);
    }),
  );
}
