import { useCallback, useEffect, useState } from "react";
import { toast } from "sonner";

export function useKeyboardListener() {
  const [listeningToShortcut, setListeningToShortcut] = useState(false);
  const [listenedShortcut, setListenedShortcut] = useState<string[]>([]);
  const [hasModifier, setHasModifier] = useState(false);
  const [hasKey, setHasKey] = useState(false);

  const shortcutListener = useCallback((event: KeyboardEvent) => {
    event.preventDefault();
    if (event.repeat) {
      return;
    }

    const keys = [];
    let localHasKey = false;
    let localHasModifier = false;

    // Check for modifier keys
    if (event.metaKey) {
      keys.push("Meta");
      localHasModifier = true;
    }
    if (event.ctrlKey) {
      keys.push("Control");
      localHasModifier = true;
    }
    if (event.shiftKey) {
      keys.push("Shift");
      localHasModifier = true;
    }
    if (event.altKey) {
      keys.push("Alt");
      localHasModifier = true;
    }

    // Only include non-modifier keys as the final main key of the shortcut
    if (!modifierKeys.has(event.key)) {
      if (event.code in codeMapping) {
        keys.push(codeMapping[event.code as keyof typeof codeMapping]);
        localHasKey = true;
      } else {
        // Warn and reset upon encountering an invalid key; this mean that the shortcut
        // contains some key that is neither a modifier key nor an allowed main key
        toast.error(`Key "${event.key}" is not allowed.`);
        setListenedShortcut([]);
        setHasKey(false);
        setHasModifier(false);
        return;
      }
    }

    // Update states with the final key combination
    setListenedShortcut(keys);
    setHasKey(localHasKey);
    setHasModifier(localHasModifier);
  }, []);

  /**
   * Cleanup the shortcut listener.
   *
   * This stops listening (and thus triggers the event listener cleanup), empties up the
   * listened shortcut keys, and resets the relevant states.
   */
  const cleanup = useCallback(() => {
    setListeningToShortcut(false);
    setListenedShortcut([]);
    setHasKey(false);
    setHasModifier(false);
  }, []);

  // Listen to the shortcut when listening toggle is on
  useEffect(() => {
    if (listeningToShortcut) {
      window.addEventListener("keydown", shortcutListener);
    } else {
      window.removeEventListener("keydown", shortcutListener);
    }

    return () => {
      window.removeEventListener("keydown", shortcutListener);
    };
  }, [listeningToShortcut]);

  return {
    listeningToShortcut,
    setListeningToShortcut,
    listenedShortcut,
    hasModifier,
    hasKey,
    cleanup,
  };
}

const codeMapping = {
  Digit0: "0",
  Digit1: "1",
  Digit2: "2",
  Digit3: "3",
  Digit4: "4",
  Digit5: "5",
  Digit6: "6",
  Digit7: "7",
  Digit8: "8",
  Digit9: "9",
  KeyA: "A",
  KeyB: "B",
  KeyC: "C",
  KeyD: "D",
  KeyE: "E",
  KeyF: "F",
  KeyG: "G",
  KeyH: "H",
  KeyI: "I",
  KeyJ: "J",
  KeyK: "K",
  KeyL: "L",
  KeyM: "M",
  KeyN: "N",
  KeyO: "O",
  KeyP: "P",
  KeyQ: "Q",
  KeyR: "R",
  KeyS: "S",
  KeyT: "T",
  KeyU: "U",
  KeyV: "V",
  KeyW: "W",
  KeyX: "X",
  KeyY: "Y",
  KeyZ: "Z",
};

const modifierKeys = new Set(["Alt", "Control", "Meta", "Shift"]);
