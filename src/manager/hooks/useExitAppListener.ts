import { useEffect, useRef } from "react";
import { listenToExitAppOnce } from "../../core/events";
import { invokeExitApp } from "../../core/commands";
import { AppSettings } from "../../types";
import { WidgetsState } from "./useWidgets";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";

export function useExitAppListener(
  appSettings: AppSettings,
  widgets: WidgetsState,
  ready: ReadyCallback,
) {
  const isReady = useRef(false);

  useEffect(() => {
    const unlisten = listenToExitAppOnce(() => {
      const widgetSettingsMap = Object.fromEntries(
        Object.entries(widgets).map(([id, { settings }]) => [id, settings]),
      );
      const settings = { app: appSettings, widgets: widgetSettingsMap };
      invokeExitApp({ settings }).catch(console.error);
    });

    if (!isReady.current) {
      ready(ListenerKeys.EXIT_APP);
      isReady.current = true;
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [appSettings, widgets]);
}
