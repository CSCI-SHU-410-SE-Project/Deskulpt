import { useEffect } from "react";
import { listenToExitApp } from "../../events";
import { invokeExitApp } from "../../commands";
import { ManagerWidgetState } from "../../types/frontend";
import { IdMap, ThemeAppearance } from "../../types/backend";

export default function useExitAppListener(
  toggleShortcut: string | null,
  themeAppearance: ThemeAppearance,
  managerWidgetStates: IdMap<ManagerWidgetState>,
) {
  useEffect(() => {
    const unlisten = listenToExitApp(() => {
      const widgetSettings = Object.fromEntries(
        Object.entries(managerWidgetStates).map(([widgetId, { setting }]) => [
          widgetId,
          setting,
        ]),
      );
      invokeExitApp({ toggleShortcut, themeAppearance, widgetSettings }).catch(
        console.error,
      );
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [toggleShortcut, themeAppearance, managerWidgetStates]);
}
