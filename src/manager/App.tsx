import { useState } from "react";
import WidgetsTab from "./tabs/WidgetsTab";
import SettingsTab from "./tabs/SettingsTab";
import AboutTab from "./tabs/AboutTab";
import useExitAppListener from "./hooks/useExitAppListener";
import useToggleShortcut from "./hooks/useToggleShortcut";
import useManagerWidgetStates from "./hooks/useManagerWidgetStates";
import useUpdateSettingsListener from "./hooks/useUpdateSettingsListener";
import { Box, Tabs, Theme } from "@radix-ui/themes";
import ManagerToaster from "./components/ManagerToaster";
import AppearanceToggler from "./components/AppearanceToggler";

/**
 * The main component of the manager window.
 */
export default function App() {
  const [appearance, setAppearance] = useState(
    window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.appearance,
  );
  const { toggleShortcut, setToggleShortcut } = useToggleShortcut(
    window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.toggleShortcut,
  );
  const { managerWidgetStates, setManagerWidgetStates, rescanAndRender } =
    useManagerWidgetStates(
      window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgetSettingsMap,
    );

  useExitAppListener(toggleShortcut, appearance, managerWidgetStates);
  useUpdateSettingsListener(setManagerWidgetStates);

  return (
    <Theme
      appearance={appearance}
      accentColor="indigo"
      grayColor="slate"
      css={{ height: "100vh" }}
    >
      <ManagerToaster appearance={appearance} />
      <AppearanceToggler
        appearance={appearance}
        setAppearance={setAppearance}
      />
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
}
