import { invoke } from "@tauri-apps/api/core";
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { useEffect, useState } from "react";

export function useToggleShortcut(initialToggleShortcut: string | null) {
  const [toggleShortcut, setToggleShortcut] = useState<string | null>(
    initialToggleShortcut,
  );

  useEffect(() => {
    if (toggleShortcut === null) {
      return;
    }

    register(toggleShortcut, () => {
      invoke<null>("toggle_click_through").catch(console.error);
    }).catch((err) => {
      console.error(err);
      setToggleShortcut(null); // Reset to null if shortcut registration fails
    });

    return () => {
      unregister(toggleShortcut).catch(console.error);
    };
  }, [toggleShortcut]);

  return { toggleShortcut, setToggleShortcut };
}
