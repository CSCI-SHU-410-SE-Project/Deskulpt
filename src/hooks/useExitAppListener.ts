import { useEffect } from "react";
import { listenToExitApp } from "../events";
import { invokeExitApp } from "../commands";
import { ManagerWidgetState } from "../types";

export function useExitAppListener(
  toggleShortcut: string | null,
  managerWidgetStates: Record<string, ManagerWidgetState>,
) {
  useEffect(() => {
    const unlisten = listenToExitApp(() => {
      const widgetSettings = Object.fromEntries(
        Object.entries(managerWidgetStates).map(([widgetId, { setting }]) => [
          widgetId,
          setting,
        ]),
      );
      invokeExitApp({ toggleShortcut, widgetSettings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [toggleShortcut, managerWidgetStates]);
}
