import { useEffect, useState } from "react";
import { invokeRegisterToggleShortcut } from "../commands";

export function useToggleShortcut(initialToggleShortcut: string | null) {
  const [toggleShortcut, setToggleShortcut] = useState<string | null>(
    initialToggleShortcut,
  );

  useEffect(() => {
    if (toggleShortcut === null) {
      return;
    }

    invokeRegisterToggleShortcut(toggleShortcut, false).catch(console.error);
    console.log("Registered shortcut", toggleShortcut);

    return () => {
      if (toggleShortcut !== null) {
        console.log("Unregistering shortcut", toggleShortcut);
        invokeRegisterToggleShortcut(toggleShortcut, true).catch(console.error);
      }
    };
  }, [toggleShortcut]);

  return { toggleShortcut, setToggleShortcut };
}
