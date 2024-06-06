import { Dispatch, SetStateAction } from "react";
import { DataList, Flex } from "@radix-ui/themes";
import SettingToggleShortcut from "../components/SettingToggleShortcut";
import Shortcut from "../components/Shortcut";

export interface SettingsTabProps {
  toggleShortcut: string | null;
  setToggleShortcut: Dispatch<SetStateAction<string | null>>;
}

/**
 * The global settings tab in the manager.
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
