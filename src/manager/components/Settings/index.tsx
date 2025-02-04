import { DataList, Flex } from "@radix-ui/themes";
import SettingToggleShortcut from "./SettingToggleShortcut";
import Shortcut from "./Shortcut";
import { useAppSettingsStore } from "../../hooks";
import { memo } from "react";

const SettingsTab = memo(() => {
  const toggleCanvas = useAppSettingsStore(
    (state) => state.shortcuts.toggleCanvas,
  );
  const shortcutKeys = toggleCanvas?.split("+") ?? [];

  return (
    <DataList.Root size="2" mx="3" my="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item align="center">
        <DataList.Label>Toggle shortcut</DataList.Label>
        <DataList.Value>
          <Flex align="center" justify="between" width="100%">
            <Shortcut keys={shortcutKeys} gap="1" />
            <SettingToggleShortcut shortcut={toggleCanvas} />
          </Flex>
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
});

export default SettingsTab;
