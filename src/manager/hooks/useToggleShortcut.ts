import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { invokeUpdateShortcut } from "../../commands";

export interface UseToggleShortcutOutput {
  /** The current toggle shortcut. */
  toggleShortcut: string | null;
  /** Setter for the toggle shortcut state. */
  setToggleShortcut: Dispatch<SetStateAction<string | null>>;
}

/**
 * The hook for managing the toggle shortcut.
 *
 * This hook is responsible for registering and unregistering the toggle shortcut on
 * shortcut change. Just use the setter it returns to change the toggle shortcut and
 * it will handle the rest.
 *
 * @param initialToggleShortcut The initial toggle shortcut to use.
 */
export default function useToggleShortcut(): UseToggleShortcutOutput {
  const [toggleShortcut, setToggleShortcut] = useState(
    window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.app.shortcuts
      .toggleCanvas,
  );

  useEffect(() => {
    invokeUpdateShortcut({
      key: "toggleCanvas",
      oldShortcut: null,
      newShortcut: toggleShortcut,
    }).catch(console.error);

    return () => {
      invokeUpdateShortcut({
        key: "toggleCanvas",
        oldShortcut: toggleShortcut,
        newShortcut: null,
      }).catch(console.error);
    };
  }, [toggleShortcut]);

  return { toggleShortcut, setToggleShortcut };
}
