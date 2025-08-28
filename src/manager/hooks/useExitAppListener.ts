import { useEffect } from "react";
import { commands } from "../../core";
import { useWidgetsStore } from "./useWidgetsStore";
import { useAppSettingsStore } from "./useAppSettingsStore";
import { ExitAppEventAPI } from "../../bindings/events";

export function useExitAppListener() {
  useEffect(() => {
    const unlisten = ExitAppEventAPI.listen(() => {
      const settings = {
        app: useAppSettingsStore.getState(),
        widgets: Object.fromEntries(
          Object.entries(useWidgetsStore.getState().widgets).map(
            ([id, { settings }]) => [id, settings],
          ),
        ),
      };
      commands.exitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
