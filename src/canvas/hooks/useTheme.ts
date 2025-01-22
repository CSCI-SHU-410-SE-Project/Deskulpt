import { useEffect, useState } from "react";
import { Theme } from "../../types/backend";
import { listenToSwitchTheme } from "../../core/events";

export function useTheme() {
  const [theme, setTheme] = useState<Theme>("dark");

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
