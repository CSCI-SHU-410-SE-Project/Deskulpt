import { useEffect, useState } from "react";
import { listenToSwitchTheme } from "../../events";

/**
 * Handle the theme of the canvas.
 *
 * This hook works by listening to the "switch-theme" event and updating the
 * theme state accordingly.
 *
 * @returns The current theme.
 */
export default function useAppearanceListener() {
  const [theme, setTheme] = useState(
    window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.app.theme,
  );

  useEffect(() => {
    const unlisten = listenToSwitchTheme((event) => {
      setTheme(event.payload);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return theme;
}
