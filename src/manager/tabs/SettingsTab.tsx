import { Dispatch, SetStateAction } from "react";
import { DataList, Flex } from "@radix-ui/themes";
import SettingToggleShortcut from "../components/SettingToggleShortcut";
import Shortcut from "../components/Shortcut";

export interface SettingsTabProps {
  /** The current toggle shortcut. */
  toggleShortcut?: string;
  /** Setter for the toggle shortcut state. */
  setToggleShortcut: Dispatch<SetStateAction<string | undefined>>;
}

/**
 * The global settings tab in the manager.
 *
 * This tab is rendered as a data list with some margin. It contains the settings and
 * setters for the global settings, which include the toggle shortcut.
 */
export default function SettingsTab({
  toggleShortcut,
  setToggleShortcut,
}: SettingsTabProps) {
  const shortcutKeys = toggleShortcut?.split("+") ?? [];

  return (
    <DataList.Root size="2" mx="3" my="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item align="center">
        <DataList.Label>Toggle shortcut</DataList.Label>
        <DataList.Value>
          <Flex align="center" justify="between" width="100%">
            <Shortcut keys={shortcutKeys} gap="1" />
            <SettingToggleShortcut setToggleShortcut={setToggleShortcut} />
          </Flex>
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
}
