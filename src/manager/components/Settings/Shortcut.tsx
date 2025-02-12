import {
  Button,
  Flex,
  IconButton,
  Kbd,
  Popover,
  Text,
  TextField,
} from "@radix-ui/themes";
import {
  FocusEvent,
  KeyboardEvent as ReactKeyboardEvent,
  memo,
  useCallback,
  useRef,
  useState,
} from "react";
import { FaEdit } from "react-icons/fa";
import { MdClear } from "react-icons/md";
import { Shortcuts } from "../../../types";
import { updateShortcut, useAppSettingsStore } from "../../hooks";
import { toast } from "sonner";

// https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
const KEYCODES = {
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
  Numpad0: "0",
  Numpad1: "1",
  Numpad2: "2",
  Numpad3: "3",
  Numpad4: "4",
  Numpad5: "5",
  Numpad6: "6",
  Numpad7: "7",
  Numpad8: "8",
  Numpad9: "9",
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

const INVALID_KEYCODES = {
  Minus: "-",
  Equal: "=",
  Backspace: "\u232b",
  BracketLeft: "[",
  BracketRight: "]",
  Enter: "\u23ce",
  Semicolon: ";",
  Quote: "'",
  Backquote: "`",
  Backslash: "\\",
  Comma: ",",
  Period: ".",
  Slash: "/",
  Space: "\u2423",
  CapsLock: "\u21ea",
  Home: "\u21f1",
  End: "\u21f2",
  PageUp: "\u21de",
  PageDown: "\u21df",
  ArrowUp: "\u2191",
  ArrowDown: "\u2193",
  ArrowLeft: "\u2190",
  ArrowRight: "\u2192",
  Insert: "\u2380",
  Delete: "\u2326",
  NumpadAdd: "+",
  NumpadSubtract: "-",
  NumpadMultiply: "*",
  NumpadDivide: "/",
  NumpadDecimal: ".",
  NumpadEnter: "\u23ce",
  NumpadEqual: "=",
  NumpadComma: ",",
};

const MODIFIERS = {
  Alt: "Alt",
  Ctrl: "Ctrl",
  Meta: "Super", // Tauri global shortcut plugin allows only "SUPER"
  Shift: "Shift",
};

const INITIAL_PLACEHOLDER = "Shortcut disabled";

interface Props {
  shortcutKey: keyof Shortcuts;
}

const ShortcutAction = memo(({ shortcutKey }: Props) => {
  const shortcut = useAppSettingsStore((state) => state.shortcuts[shortcutKey]);

  const inputRef = useRef<HTMLInputElement>(null);
  const clearButtonRef = useRef<HTMLButtonElement>(null);
  const confirmButtonRef = useRef<HTMLButtonElement>(null);

  const [value, setValue] = useState(shortcut ?? "");
  const [placeholder, setPlaceholder] = useState(INITIAL_PLACEHOLDER);
  const [isValid, setIsValid] = useState(true);

  const handleFocus = useCallback(() => {
    setPlaceholder("Press key combination...");
  }, []);

  const handleBlur = useCallback(
    (event: FocusEvent) => {
      // Prevent resetting when clicking on the clear or confirm button,
      // otherwise they cannot access the correct value
      if (
        event.relatedTarget === confirmButtonRef.current ||
        event.relatedTarget === clearButtonRef.current
      ) {
        return;
      }
      setValue(shortcut ?? "");
      setPlaceholder(INITIAL_PLACEHOLDER);
    },
    [shortcut],
  );

  const handleOpenChange = useCallback(
    (open: boolean) => {
      if (open) {
        // Reset states on popover open
        setValue(shortcut ?? "");
        setPlaceholder(INITIAL_PLACEHOLDER);
        setIsValid(true);
      }
    },
    [shortcut],
  );

  const handleKeyDown = useCallback((event: ReactKeyboardEvent) => {
    if (event.key === "Tab" || event.repeat) {
      // Ignore key repeats for performance; ignore tab key for keyboard
      // accessibility, i.e., on should be able to use it to navigate to the
      // close and confirm buttons instead of being trapped in the input field
      return;
    }
    event.preventDefault();

    const keys = [];
    let localHasKey = false;
    let localHasModifier = false;

    // Check for modifier keys
    if (event.metaKey) {
      keys.push(MODIFIERS.Meta);
      localHasModifier = true;
    }
    if (event.ctrlKey) {
      keys.push(MODIFIERS.Ctrl);
      localHasModifier = true;
    }
    if (event.shiftKey) {
      keys.push(MODIFIERS.Shift);
      localHasModifier = true;
    }
    if (event.altKey) {
      keys.push(MODIFIERS.Alt);
      localHasModifier = true;
    }

    // Only include non-modifier keys as the final main key of the shortcut
    if (event.code in KEYCODES) {
      keys.push(KEYCODES[event.code as keyof typeof KEYCODES]);
      localHasKey = true;
    } else if (event.code in INVALID_KEYCODES) {
      keys.push(INVALID_KEYCODES[event.code as keyof typeof INVALID_KEYCODES]);
    }

    setValue(keys.join(" + "));
    setIsValid(localHasKey && localHasModifier);
  }, []);

  const confirmAction = useCallback(() => {
    updateShortcut(shortcutKey, shortcut, value === "" ? null : value)
      .then(() => {
        setPlaceholder(INITIAL_PLACEHOLDER);
        setIsValid(true);
        toast.success("Shortcut updated.");
      })
      .catch(() => {
        toast.error("Failed to update shortcut.");
      });
  }, [shortcutKey, shortcut, value]);

  const clearAction = useCallback(() => {
    if (inputRef.current === null) {
      setPlaceholder(INITIAL_PLACEHOLDER);
    } else {
      inputRef.current.focus();
    }
    setValue("");
    setIsValid(true);
  }, []);

  return (
    <Flex align="center" justify="end" gap="4">
      {shortcut === null ? (
        <Text color="gray">Disabled</Text>
      ) : (
        <Kbd size="3">{shortcut}</Kbd>
      )}
      <Popover.Root onOpenChange={handleOpenChange}>
        <Popover.Trigger>
          <Button size="1" variant="surface">
            <FaEdit /> Edit
          </Button>
        </Popover.Trigger>
        <Popover.Content size="1" width="400px">
          <Text size="2" as="div" mb="3">
            A keyboard shortcut must contain at least one modifier key (Ctrl,
            Shift, Alt) and one alphanumeric key. A red border means that the
            shortcut is invalid.
          </Text>
          <Flex gap="3" align="center">
            <TextField.Root
              ref={inputRef}
              size="1"
              variant="surface"
              readOnly
              value={value}
              placeholder={placeholder}
              onFocus={handleFocus}
              onBlur={handleBlur}
              onKeyDown={handleKeyDown}
              css={{
                width: "240px",
                fontSize: "var(--font-size-2)",
                paddingLeft: "var(--space-1)",
                "> input": { cursor: "text" },
                "--text-field-focus-color": isValid
                  ? "var(--accent-8)"
                  : "var(--red-8)",
              }}
            >
              <TextField.Slot side="right">
                <IconButton
                  ref={clearButtonRef}
                  size="1"
                  variant="ghost"
                  disabled={value === ""}
                  onClick={clearAction}
                >
                  <MdClear size="15" />
                </IconButton>
              </TextField.Slot>
            </TextField.Root>
            <Popover.Close>
              <Button
                ref={confirmButtonRef}
                size="1"
                variant="surface"
                disabled={!isValid || (shortcut ?? "") === value}
                onClick={confirmAction}
              >
                {value === "" ? "Disable" : "Confirm"}
              </Button>
            </Popover.Close>
          </Flex>
        </Popover.Content>
      </Popover.Root>
    </Flex>
  );
});

export default ShortcutAction;
