import { useEffect, useState } from "react";
import { ThemeAppearance } from "../../types/backend";
import { listenToSwitchThemeAppearance } from "../../events";

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
