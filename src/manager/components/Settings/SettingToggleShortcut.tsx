import {
  Badge,
  Blockquote,
  Button,
  Checkbox,
  Em,
  Flex,
  Popover,
  Strong,
  Text,
} from "@radix-ui/themes";
import { useEffect, useState } from "react";
import Shortcut from "./Shortcut";
import { FaEdit } from "react-icons/fa";
import { updateShortcut, useKeyboardListener } from "../../hooks";

interface SettingToggleShortcutProps {
  shortcut: string | null;
}

const SettingToggleShortcut = ({ shortcut }: SettingToggleShortcutProps) => {
  const [popoverOpen, setPopoverOpen] = useState(false);
  const [disableShortcut, setDisableShortcut] = useState(false);

  const {
    listeningToShortcut,
    setListeningToShortcut,
    listenedShortcut,
    hasModifier,
    hasKey,
    cleanup,
  } = useKeyboardListener();

  // Cleanup when shortcut changes from enabled to disabled
  useEffect(() => {
    if (disableShortcut) {
      cleanup();
    }
  }, [disableShortcut, cleanup]);

  // Cleanup and reset states when popover is opened/closed
  useEffect(() => {
    setDisableShortcut(false);
    cleanup();
  }, [popoverOpen, cleanup]);

  return (
    <Popover.Root
      open={popoverOpen}
      onOpenChange={(isOpen) => setPopoverOpen(isOpen)}
    >
      <Popover.Trigger>
        <Button
          size="1"
          variant="surface"
          color="gray"
          highContrast
          onClick={() => setPopoverOpen(true)}
        >
          <FaEdit /> Edit
        </Button>
      </Popover.Trigger>
      <Popover.Content
        side="left"
        size="1"
        width="500px"
        onInteractOutside={(e) => e.preventDefault()}
        onPointerDownOutside={(e) => e.preventDefault()}
        onFocusOutside={(e) => e.preventDefault()}
        asChild
      >
        <Flex direction="column" gap="3">
          {/* Introduction */}
          <Blockquote size="1" color="gray">
            The toggle shortcut is used for toggling the floating/sinking state
            of the canvas, equivalent to the &quot;Float/Sink&quot; option in
            the tray menu. Widgets are not interactable when the canvas is
            floated, and the desktop is not interactable when the canvas is
            sunk.
          </Blockquote>
          {/* Decision whether to disable the shortcut */}
          <Text size="1">
            <Flex gap="2" align="center">
              <Checkbox
                size="1"
                checked={disableShortcut}
                onCheckedChange={(checked) => {
                  setDisableShortcut(
                    checked === "indeterminate" ? true : checked,
                  );
                }}
              />
              <Text>
                Disable the toggle shortcut.{" "}
                <Em>Use the tray to float/sink the canvas instead.</Em>
              </Text>
            </Flex>
          </Text>
          {/* Usage guidance */}
          <Text size="1">
            Start by clicking the <Strong>Start Listening</Strong> button and
            pressing the desired shortcut.
          </Text>
          {/* Shortcut setting panel */}
          {!disableShortcut && (
            <Shortcut
              keys={listenedShortcut}
              height="100px"
              justify="center"
              gap="3"
              size="3"
              css={{
                borderRadius: "var(--radius-2)",
                backgroundImage: `url("data:image/svg+xml,%3Csvg width='6' height='6' viewBox='0 0 6 6' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='%239C92AC' fill-opacity='0.2' fill-rule='evenodd'%3E%3Cpath d='M5 0h1L0 6V5zM6 5v1H5z'/%3E%3C/g%3E%3C/svg%3E")`,
              }}
            />
          )}
          {/* Action buttons */}
          <Flex justify="end" gap="2">
            {!disableShortcut && (
              <Button
                size="1"
                variant="surface"
                color="gray"
                highContrast
                onClick={() => {
                  setListeningToShortcut((prev) => !prev);
                }}
              >
                {listeningToShortcut ? "Stop listening" : "Start listening"}
              </Button>
            )}
            <Button
              size="1"
              variant="surface"
              color="gray"
              highContrast
              disabled={!disableShortcut && (!hasModifier || !hasKey)}
              onClick={() => {
                updateShortcut(
                  "toggleCanvas",
                  shortcut,
                  disableShortcut ? null : listenedShortcut.join("+"),
                );
                setPopoverOpen(false);
              }}
            >
              Confirm
            </Button>
            <Popover.Close>
              <Button size="1" variant="surface" color="red">
                Cancel
              </Button>
            </Popover.Close>
          </Flex>
          {/* Real-time validation messages */}
          {!disableShortcut && (
            <Flex direction="column" gap="1">
              {!hasModifier && (
                <Text size="1">
                  <Badge color="red">Invalid</Badge> Shortcut must contain at
                  least one modifier.
                </Text>
              )}
              {!hasKey && (
                <Text size="1">
                  <Badge color="red">Invalid</Badge> Shortcut must contain an
                  alphanumerical main key.
                </Text>
              )}
            </Flex>
          )}
        </Flex>
      </Popover.Content>
    </Popover.Root>
  );
};

export default SettingToggleShortcut;
