import { useEffect } from "react";
import { listenToExitApp } from "../../core/events";
import { invokeExitApp } from "../../core/commands";
import { ManagerWidgetState } from "../../types/frontend";
import { Theme } from "../../types/backend";

export function useExitAppListener(
  toggleShortcut: string | undefined,
  theme: Theme,
  managerWidgetStates: Record<string, ManagerWidgetState>,
) {
  useEffect(() => {
    const unlisten = listenToExitApp(() => {
      const widgetSettingsMap = Object.fromEntries(
        Object.entries(managerWidgetStates).map(([id, { settings }]) => [
          id,
          settings,
        ]),
      );
      const settings = { toggleShortcut, theme, widgetSettingsMap };
      invokeExitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [toggleShortcut, theme, managerWidgetStates]);
}
