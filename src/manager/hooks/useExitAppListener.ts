import { useEffect } from "react";
import { commands, events } from "../../core";
import { WidgetSettings } from "../../types";
import { useWidgetsStore } from "./useWidgetsStore";
import { useAppSettingsStore } from "./useAppSettingsStore";

export function useExitAppListener() {
  useEffect(() => {
    const unlisten = events.exitApp.on(() => {
      const widgets = useWidgetsStore.getState().widgets.reduce(
        (acc, { id, settings }) => {
          acc[id] = settings;
          return acc;
        },
        {} as Record<string, WidgetSettings>,
      );

      const app = useAppSettingsStore.getState();
      const settings = { app, widgets };
      commands.exitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
