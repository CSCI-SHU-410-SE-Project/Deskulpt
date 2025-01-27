import { useEffect, useState } from "react";
import { listenToSwitchTheme } from "../../core/events";
import { Theme } from "../../types";

export function useTheme() {
  const [theme, setTheme] = useState(
    window.__DESKULPT__.initialSettings.app.theme,
  );

  useEffect(() => {
    const unlisten = listenToSwitchTheme(() => {
      setTheme((prevTheme) =>
        prevTheme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT,
      );
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return theme;
}
