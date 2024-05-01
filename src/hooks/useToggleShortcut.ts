import { invoke } from "@tauri-apps/api/core";
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { useEffect, useState } from "react";

export function useToggleShortcut(initialToggleShortcut: string) {
  const [toggleShortcut, setToggleShortcut] = useState<string>(initialToggleShortcut);

  useEffect(() => {
    // Register the global shortcut to toggle click-through
    register(toggleShortcut, () => {
      invoke<null>("toggle_click_through").catch(console.error);
    }).catch(console.error);

    return () => {
      unregister(toggleShortcut).catch(console.error);
    };
  }, [toggleShortcut]);

  return { toggleShortcut, setToggleShortcut };
}
