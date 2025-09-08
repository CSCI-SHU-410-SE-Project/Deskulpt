import { useEffect } from "react";
import { useWidgetsStore } from "./useWidgetsStore";
import { useAppSettingsStore } from "./useAppSettingsStore";
import { commands, events } from "../../bindings";

export function useExitAppListener() {
  useEffect(() => {
    const unlisten = events.exitAppEvent.listen(() => {
      const settings = {
        ...useAppSettingsStore.getState(),
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
