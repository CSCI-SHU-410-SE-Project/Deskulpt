import { DataList, Flex } from "@radix-ui/themes";
import { SettingToggleShortcut, Shortcut } from "../components";
import { AppSettings } from "../../types";
import { UpdateShortcutsCallback } from "../hooks";

interface Props {
  appSettings: AppSettings;
  updateShortcuts: UpdateShortcutsCallback;
}

/**
 * The global settings tab in the manager.
 *
 * This tab is rendered as a data list with some margin. It contains the settings and
 * setters for the global settings, which include the toggle shortcut.
 */
export default ({ appSettings, updateShortcuts }: Props) => {
  const shortcutKeys = appSettings.shortcuts.canvasToggle?.split("+") ?? [];

  return (
    <DataList.Root size="2" mx="3" my="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item align="center">
        <DataList.Label>Toggle shortcut</DataList.Label>
        <DataList.Value>
          <Flex align="center" justify="between" width="100%">
            <Shortcut keys={shortcutKeys} gap="1" />
            <SettingToggleShortcut
              shortcuts={appSettings.shortcuts}
              updateShortcuts={updateShortcuts}
            />
          </Flex>
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
};
