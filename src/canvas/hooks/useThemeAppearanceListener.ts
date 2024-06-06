import { useEffect, useState } from "react";
import { ThemeAppearance } from "../../types/backend";
import { listenToSwitchThemeAppearance } from "../../events";

/**
 * Handle the theme appearance of the canvas.
 *
 * This hook works by listening to the "switch-theme-appearance" event and updating the
 * appearance state accordingly. The appearance is initialized to "dark".
 *
 * @returns The current theme appearance.
 */
export default function useThemeAppearanceListener() {
  const [appearance, setThemeAppearance] = useState<ThemeAppearance>("dark");

  useEffect(() => {
    const unlisten = listenToSwitchThemeAppearance((event) => {
      setThemeAppearance(event.payload);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return appearance;
}
