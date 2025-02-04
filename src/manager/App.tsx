import { useState } from "react";
import WidgetsTab from "./tabs/WidgetsTab";
import SettingsTab from "./tabs/SettingsTab";
import AboutTab from "./tabs/AboutTab";
import useExitAppListener from "./hooks/useExitAppListener";
import useToggleShortcut from "./hooks/useToggleShortcut";
import useUpdateSettingsListener from "./hooks/useUpdateSettingsListener";
import { Box, Theme as RadixTheme, Tabs } from "@radix-ui/themes";
import ManagerToaster from "./components/ManagerToaster";
import ThemeToggler from "./components/ThemeToggler";
import useInitialRescan from "./hooks/useInitialRescan";

/**
 * The main component of the manager window.
 */
export default function App() {
  const [theme, setTheme] = useState(
    window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.app.theme,
  );
  const { toggleShortcut, setToggleShortcut } = useToggleShortcut();

  useExitAppListener(toggleShortcut, theme);
  useInitialRescan();
  useUpdateSettingsListener();

  return (
    <RadixTheme
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
                <WidgetsTab />
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
    </RadixTheme>
  );
}
