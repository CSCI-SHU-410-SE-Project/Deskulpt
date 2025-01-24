import { useCallback, useState } from "react";
import { Theme } from "../../types";
import { emitSwitchThemeToCanvas } from "../../core/events";

export type ToggleThemeCallback = () => void;

export function useTheme() {
  const [theme, setTheme] = useState(window.__DESKULPT__.initialSettings.theme);

  const toggleTheme = useCallback(() => {
    const newTheme = theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT;
    setTheme(newTheme);
    emitSwitchThemeToCanvas({ theme: newTheme }).catch(console.error);
  }, [theme]);

  return [theme, toggleTheme] as const;
}
