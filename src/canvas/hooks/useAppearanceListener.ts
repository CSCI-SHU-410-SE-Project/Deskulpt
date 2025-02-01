import { useEffect, useState } from "react";
import { listenToSwitchAppearance } from "../../events";

/**
 * Handle the theme appearance of the canvas.
 *
 * This hook works by listening to the "switch-theme-appearance" event and updating the
 * appearance state accordingly. The appearance is initialized to "dark".
 *
 * @returns The current theme appearance.
 */
export default function useAppearanceListener() {
  const [appearance, setAppearance] = useState(
    window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.appearance,
  );

  useEffect(() => {
    const unlisten = listenToSwitchAppearance((event) => {
      setAppearance(event.payload);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return appearance;
}
