import { useState } from "react";
import WidgetsTab from "./tabs/WidgetsTab";
import SettingsTab from "./tabs/SettingsTab";
import AboutTab from "./tabs/AboutTab";
import {
  useExitAppListener,
  useToggleShortcut,
  useManagerWidgetStates,
  useUpdateSettingsListener,
} from "./hooks";
import { Settings } from "../types/backend";
import { Box, Tabs, Theme } from "@radix-ui/themes";
import { ThemeToggler, ManagerToaster } from "./components";

interface Props {
  settings: Settings;
}

export default ({ settings }: Props) => {
  const [theme, setTheme] = useState(settings.theme);
  const { toggleShortcut, setToggleShortcut } = useToggleShortcut(
    settings.toggleShortcut,
  );
  const { managerWidgetStates, setManagerWidgetStates, rescanAndRender } =
    useManagerWidgetStates(settings.widgetSettingsMap);

  useExitAppListener(toggleShortcut, theme, managerWidgetStates);
  useUpdateSettingsListener(setManagerWidgetStates);

  return (
    <Theme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      css={{ height: "100vh" }}
    >
      <ManagerToaster theme={theme} />
      <ThemeToggler theme={theme} setTheme={setTheme} />
      <Tabs.Root defaultValue="widgets" asChild>
        <Box height="100%" p="2">
          <Tabs.List>
            <Tabs.Trigger value="widgets">Widgets</Tabs.Trigger>
            <Tabs.Trigger value="settings">Settings</Tabs.Trigger>
            <Tabs.Trigger value="about">About</Tabs.Trigger>
          </Tabs.List>
          {/* Tab triggers have ~40px height */}
          <Box px="1" py="3" css={{ height: "calc(100% - 40px)" }}>
            <Tabs.Content value="widgets" asChild>
              <Box height="100%">
                <WidgetsTab
                  managerWidgetStates={managerWidgetStates}
                  setManagerWidgetStates={setManagerWidgetStates}
                  rescanAndRender={rescanAndRender}
                />
              </Box>
            </Tabs.Content>
            <Tabs.Content value="settings" asChild>
              <Box height="100%">
                <SettingsTab
                  toggleShortcut={toggleShortcut}
                  setToggleShortcut={setToggleShortcut}
                />
              </Box>
            </Tabs.Content>
            <Tabs.Content value="about" asChild>
              <Box height="100%">
                <AboutTab />
              </Box>
            </Tabs.Content>
          </Box>
        </Box>
      </Tabs.Root>
    </Theme>
  );
};
