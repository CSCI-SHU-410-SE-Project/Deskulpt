import { useEffect, useState } from "react";
import { events } from "../../core";

export function useTheme() {
  const [theme, setTheme] = useState(
    window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.app.theme,
  );

  useEffect(() => {
    const unlisten = events.switchTheme.on((event) => {
      setTheme(event.payload.theme);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return theme;
}
