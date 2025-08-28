import { useEffect, useState } from "react";
import { SwitchThemeEventAPI } from "../../bindings/events";

export function useTheme() {
  const [theme, setTheme] = useState(
    window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.app.theme,
  );

  useEffect(() => {
    const unlisten = SwitchThemeEventAPI.listen((event) => {
      setTheme(event.payload);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return theme;
}
