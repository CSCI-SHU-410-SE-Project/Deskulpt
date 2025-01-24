import { useEffect, useState } from "react";
import { invokeUpdateToggleShortcut } from "../../core/commands";

export function useToggleShortcut() {
  const [toggleShortcut, setToggleShortcut] = useState(
    window.__DESKULPT__.initialSettings.toggleShortcut,
  );

  useEffect(() => {
    invokeUpdateToggleShortcut({ newShortcut: toggleShortcut }).catch(
      console.error,
    );

    return () => {
      invokeUpdateToggleShortcut({ oldShortcut: toggleShortcut }).catch(
        console.error,
      );
    };
  }, [toggleShortcut]);

  return [toggleShortcut, setToggleShortcut] as const;
}
