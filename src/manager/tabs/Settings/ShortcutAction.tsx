import {
  Button,
  Dialog,
  Flex,
  IconButton,
  Text,
  TextField,
  VisuallyHidden,
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
import { KEY_CODE_MAPPING, MODIFIER_MAPPING } from "../../consts";
import { UpdateShortcutCallback } from "../../hooks";
import { Shortcuts } from "../../../types";
import { css } from "@emotion/react";

const styles = {
  input: css({
    width: "240px",
    fontSize: "var(--font-size-2)",
    paddingLeft: "var(--space-1)",
    "> input": { cursor: "text" },
  }),
};

interface Props {
  shortcutKey: keyof Shortcuts;
  shortcut: string | null;
  updateShortcut: UpdateShortcutCallback;
}

const ShortcutAction = memo(
  ({ shortcutKey, shortcut, updateShortcut }: Props) => {
    const inputRef = useRef<HTMLInputElement>(null);
    const clearButtonRef = useRef<HTMLButtonElement>(null);
    const confirmButtonRef = useRef<HTMLButtonElement>(null);

    const [value, setValue] = useState(shortcut ?? "");
    const [placeholder, setPlaceholder] = useState("Shortcut disabled");
    const [isValid, setIsValid] = useState(true);

    const onFocus = useCallback(() => {
      setPlaceholder("Press key combination...");
    }, []);

    const onBlur = useCallback(
      (event: FocusEvent) => {
        if (
          event.relatedTarget === confirmButtonRef.current ||
          event.relatedTarget === clearButtonRef.current
        ) {
          return;
        }
        setValue(shortcut ?? "");
        setPlaceholder("Shortcut disabled");
      },
      [shortcut],
    );

    const onOpenChange = useCallback(
      (open: boolean) => {
        if (open) {
          setValue(shortcut ?? "");
          setPlaceholder("Shortcut disabled");
          setIsValid(true);
        }
      },
      [shortcut],
    );

    const onEscapeKeyDown = useCallback((event: KeyboardEvent) => {
      event.preventDefault();
    }, []);

    const confirmAction = useCallback(() => {
      updateShortcut(shortcutKey, shortcut, value === "" ? null : value);
      setPlaceholder("Shortcut disabled");
      setIsValid(true);
    }, [shortcutKey, shortcut, value, updateShortcut]);

    const clearAction = useCallback(() => {
      if (inputRef.current === null) {
        setPlaceholder("Shortcut disabled");
      } else {
        inputRef.current.focus();
      }
      setValue("");
      setIsValid(true);
    }, []);

    const onKeyDown = useCallback((event: ReactKeyboardEvent) => {
      event.preventDefault();
      if (event.repeat) {
        return;
      }

      const keys = [];
      let localHasKey = false;
      let localHasModifier = false;

      // Check for modifier keys
      if (event.metaKey) {
        keys.push(MODIFIER_MAPPING.Meta);
        localHasModifier = true;
      }
      if (event.ctrlKey) {
        keys.push(MODIFIER_MAPPING.Ctrl);
        localHasModifier = true;
      }
      if (event.shiftKey) {
        keys.push(MODIFIER_MAPPING.Shift);
        localHasModifier = true;
      }
      if (event.altKey) {
        keys.push(MODIFIER_MAPPING.Alt);
        localHasModifier = true;
      }

      // Only include non-modifier keys as the final main key of the shortcut
      if (event.code in KEY_CODE_MAPPING) {
        keys.push(
          KEY_CODE_MAPPING[event.code as keyof typeof KEY_CODE_MAPPING],
        );
        localHasKey = true;
      }

      setValue(keys.join(" + "));
      setIsValid(localHasKey && localHasModifier);
    }, []);

    return (
      <Dialog.Root onOpenChange={onOpenChange}>
        <Dialog.Trigger>
          <Button size="1" variant="surface">
            <FaEdit /> Edit
          </Button>
        </Dialog.Trigger>
        <Dialog.Content
          width="400px"
          size="2"
          onEscapeKeyDown={onEscapeKeyDown}
          aria-describedby={undefined}
        >
          <VisuallyHidden>
            <Dialog.Title>Keyboard Shortcut Setter</Dialog.Title>
          </VisuallyHidden>
          <Text size="2" as="div" mb="3">
            A keyboard shortcut must contain at least one modifier key (Ctrl,
            Shift, Alt) and one alphanumeric main key.
          </Text>
          <Text size="2" as="div" mb="3">
            A red border means that the shortcut is invalid.
          </Text>
          <Flex gap="3" align="center" justify="center" py="4">
            <TextField.Root
              ref={inputRef}
              size="1"
              variant="surface"
              readOnly
              value={value}
              placeholder={placeholder}
              onFocus={onFocus}
              onBlur={onBlur}
              onKeyDown={onKeyDown}
              css={{
                ...styles.input,
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
            <Dialog.Close>
              <Button
                ref={confirmButtonRef}
                size="1"
                variant="surface"
                disabled={!isValid || (shortcut ?? "") === value}
                onClick={confirmAction}
              >
                Confirm
              </Button>
            </Dialog.Close>
          </Flex>
        </Dialog.Content>
      </Dialog.Root>
    );
  },
);

export default ShortcutAction;
