import { useEffect, useState } from "react";
import { listenToToggleTheme } from "../../events";
import { Theme } from "../../types/backend";

export function useTheme() {
  const [theme, setTheme] = useState(
    window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.app.theme,
  );

  useEffect(() => {
    const unlisten = listenToToggleTheme(() => {
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
