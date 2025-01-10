import { Dispatch, SetStateAction, useEffect, useState } from "react";
import { invokeUpdateToggleShortcut } from "../../commands";

export interface UseToggleShortcutOutput {
  /** The current toggle shortcut. */
  toggleShortcut?: string;
  /** Setter for the toggle shortcut state. */
  setToggleShortcut: Dispatch<SetStateAction<string | undefined>>;
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
  initialToggleShortcut?: string,
): UseToggleShortcutOutput {
  const [toggleShortcut, setToggleShortcut] = useState(initialToggleShortcut);

  useEffect(() => {
    invokeUpdateToggleShortcut({ newShortcut: toggleShortcut }).catch(console.error);

    return () => {
      invokeUpdateToggleShortcut({ oldShortcut: toggleShortcut }).catch(console.error);
    };
  }, [toggleShortcut]);

  return { toggleShortcut, setToggleShortcut };
}
