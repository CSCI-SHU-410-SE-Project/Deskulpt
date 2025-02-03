import { useEffect } from "react";
import { listenToExitAppOnce } from "../../core/events";
import { invokeExitApp } from "../../core/commands";
import { AppSettings, WidgetSettings } from "../../types";
import { WidgetsState } from "./useWidgets";

export function useExitAppListener(
  appSettings: AppSettings,
  widgets: WidgetsState,
) {
  useEffect(() => {
    const unlisten = listenToExitAppOnce(() => {
      const widgetSettingsMap = widgets.reduce(
        (acc, { id, settings }) => {
          acc[id] = settings;
          return acc;
        },
        {} as Record<string, WidgetSettings>,
      );

      const settings = { app: appSettings, widgets: widgetSettingsMap };
      invokeExitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [appSettings, widgets]);
}
