import { useEffect } from "react";
import { listenToExitApp } from "../../events";
import { invokeExitApp } from "../../commands";
import { ManagerWidgetState } from "../../types/frontend";
import { Theme } from "../../types/backend";

/**
 * Listen and react to the "exit-app" event.
 *
 * Upon receiving the event, the hook will collect all current states that needs to be
 * persisted and invoke the backend to persist them and exit the app.
 *
 * @param theme The current theme.
 * @param managerWidgetStates The current manager widget states.
 * @param toggleShortcut The current toggle shortcut.
 */
export default function useExitAppListener(
  toggleShortcut: string | null,
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
      const settings = {
        app: { theme, shortcuts: { toggleCanvas: toggleShortcut } },
        widgets: widgetSettingsMap,
      };
      invokeExitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [toggleShortcut, theme, managerWidgetStates]);
}
