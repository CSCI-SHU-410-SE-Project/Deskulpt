import { useEffect } from "react";
import { listenToExitAppOnce } from "../../core/events";
import { invokeExitApp } from "../../core/commands";
import { AppSettings } from "../../types";
import { WidgetsState } from "./useWidgets";

export function useExitAppListener(
  appSettings: AppSettings,
  widgets: WidgetsState,
) {
  useEffect(() => {
    const unlisten = listenToExitAppOnce(() => {
      const widgetSettingsMap = Object.fromEntries(
        Object.entries(widgets).map(([id, { settings }]) => [id, settings]),
      );
      const settings = { app: appSettings, widgets: widgetSettingsMap };
      invokeExitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [appSettings, widgets]);
}
