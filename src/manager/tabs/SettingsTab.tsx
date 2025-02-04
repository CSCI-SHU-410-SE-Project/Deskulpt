import { DataList, Flex } from "@radix-ui/themes";
import SettingToggleShortcut from "../components/SettingToggleShortcut";
import Shortcut from "../components/Shortcut";
import { useAppSettingsStore } from "../hooks";

const SettingsTab = () => {
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
};

export default SettingsTab;
