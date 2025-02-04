import { useEffect } from "react";
import { commands, events } from "../../core";
import { Theme, WidgetSettings } from "../../types";
import { useWidgetsStore } from "./useWidgetsStore";

export default function useExitAppListener(
  toggleShortcut: string | null,
  theme: Theme,
) {
  useEffect(() => {
    const unlisten = events.exitApp.on(() => {
      const widgets = useWidgetsStore.getState().widgets.reduce(
        (acc, { id, settings }) => {
          acc[id] = settings;
          return acc;
        },
        {} as Record<string, WidgetSettings>,
      );

      const settings = {
        app: { theme, shortcuts: { toggleCanvas: toggleShortcut } },
        widgets,
      };
      commands.exitApp({ settings }).catch(console.error);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [toggleShortcut, theme]);
}
