import { useEffect } from "react";
import { listenToExitApp } from "../../events";
import { invokeExitApp } from "../../commands";
import { ManagerWidgetState } from "../../types/frontend";
import { ThemeAppearance } from "../../types/backend";

/**
 * Listen and react to the "exit-app" event.
 *
 * Upon receiving the event, the hook will collect all current states that needs to be
 * persisted and invoke the backend to persist them and exit the app.
 *
 * @param toggleShortcut The current toggle shortcut.
 * @param themeAppearance The current theme appearance.
 * @param managerWidgetStates The current manager widget states.
 */
export default function useExitAppListener(
  toggleShortcut: string | null,
  themeAppearance: ThemeAppearance,
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
      invokeExitApp({ toggleShortcut, themeAppearance, widgetSettings }).catch(
        console.error,
      );
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [toggleShortcut, themeAppearance, managerWidgetStates]);
}
