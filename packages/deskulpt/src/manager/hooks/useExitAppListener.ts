import { useEffect } from "react";
import { useWidgetsStore } from "./useWidgetsStore";
import { useAppSettingsStore } from "./useAppSettingsStore";
import { commands, events } from "../../bindings";

export function useExitAppListener() {
  useEffect(() => {
    const unlisten = events.exitApp.listen(() => {
      const settings = {
        app: useAppSettingsStore.getState(),
        widgets: Object.fromEntries(
          Object.entries(useWidgetsStore.getState().widgets).map(
            ([id, { settings }]) => [id, settings],
          ),
        ),
      };
      commands.core.exitApp(settings).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
