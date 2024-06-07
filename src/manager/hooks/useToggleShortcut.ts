import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { invokeRegisterToggleShortcut } from "../../commands";

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
export default function useToggleShortcut(
  initialToggleShortcut: string | null,
): UseToggleShortcutOutput {
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
