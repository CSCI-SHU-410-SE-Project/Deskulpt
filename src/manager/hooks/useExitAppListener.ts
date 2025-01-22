import { useEffect } from "react";
import { listenToExitApp } from "../../core/events";
import { invokeExitApp } from "../../core/commands";
import { ManagerWidgetState } from "../../types/frontend";
import { Appearance } from "../../types/backend";

/**
 * Listen and react to the "exit-app" event.
 *
 * Upon receiving the event, the hook will collect all current states that needs to be
 * persisted and invoke the backend to persist them and exit the app.
 *
 * @param appearance The current theme appearance.
 * @param managerWidgetStates The current manager widget states.
 * @param toggleShortcut The current toggle shortcut.
 */
export function useExitAppListener(
  toggleShortcut: string | undefined,
  appearance: Appearance,
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
      const settings = { toggleShortcut, appearance, widgetSettingsMap };
      invokeExitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [toggleShortcut, appearance, managerWidgetStates]);
}
