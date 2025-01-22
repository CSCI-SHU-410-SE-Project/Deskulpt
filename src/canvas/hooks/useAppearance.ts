import { useEffect, useState } from "react";
import { Appearance } from "../../types/backend";
import { listenToSwitchAppearance } from "../../core/events";

export function useAppearance() {
  const [appearance, setAppearance] = useState<Appearance>("dark");

  useEffect(() => {
    const unlisten = listenToSwitchAppearance((event) => {
      const { appearance } = event.payload;
      setAppearance(appearance);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return appearance;
}
