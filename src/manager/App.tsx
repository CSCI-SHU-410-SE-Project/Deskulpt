import { useState } from "react";
import WidgetsTab from "./tabs/WidgetsTab";
import SettingsTab from "./tabs/SettingsTab";
import AboutTab from "./tabs/AboutTab";
import useExitAppListener from "./hooks/useExitAppListener";
import useToggleShortcut from "./hooks/useToggleShortcut";
import useManagerWidgetStates from "./hooks/useManagerWidgetStates";
import useUpdateSettingListener from "./hooks/useUpdateSettingListener";
import { GlobalSetting } from "../types/backend";
import { Box, Tabs, Theme } from "@radix-ui/themes";
import ManagerToaster from "./components/ManagerToaster";
import ThemeAppearanceToggler from "./components/ThemeAppearanceToggler";

export interface ManagerAppProps {
  /** The initial settings read from the previously saved setting file. */
  initialGlobalSetting: GlobalSetting;
}

/**
 * The main component of the manager window.
 */
export default function App({ initialGlobalSetting }: ManagerAppProps) {
  const [themeAppearance, setThemeAppearance] = useState(
    initialGlobalSetting.themeAppearance,
  );
  const { toggleShortcut, setToggleShortcut } = useToggleShortcut(
    initialGlobalSetting.toggleShortcut,
  );
  const { managerWidgetStates, setManagerWidgetStates, rescanAndRender } =
    useManagerWidgetStates(initialGlobalSetting.widgetSettings);

  useExitAppListener(toggleShortcut, themeAppearance, managerWidgetStates);
  useUpdateSettingListener(setManagerWidgetStates);

  return (
    <Theme
      appearance={themeAppearance}
      accentColor="indigo"
      grayColor="slate"
      css={{ height: "100vh" }}
    >
      <ManagerToaster themeAppearance={themeAppearance} />
      <ThemeAppearanceToggler
        themeAppearance={themeAppearance}
        setThemeAppearance={setThemeAppearance}
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
