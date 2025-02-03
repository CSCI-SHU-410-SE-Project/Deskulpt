import { useEffect, useState } from "react";
import { listenToSwitchTheme } from "../../events";

export function useTheme() {
  const [theme, setTheme] = useState(
    window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.app.theme,
  );

  useEffect(() => {
    const unlisten = listenToSwitchTheme((event) => {
      const { theme } = event.payload;
      setTheme(theme);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return theme;
}
