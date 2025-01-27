import WidgetsTab from "./tabs/WidgetsTab";
import SettingsTab from "./tabs/SettingsTab";
import AboutTab from "./tabs/AboutTab";
import {
  useAppSettings,
  useExitAppListener,
  useRescanCallback,
  useRescanInitially,
  useUpdateSettingsListener,
  useUpdateShortcutsCallback,
  useWidgets,
} from "./hooks";
import { Box, Theme as RadixTheme, Tabs } from "@radix-ui/themes";
import { ThemeToggler } from "./components";
import { Toaster } from "sonner";

export default () => {
  const [widgets, widgetsDispatch] = useWidgets();
  const [appSettings, appSettingsDispatch] = useAppSettings();
  const rescan = useRescanCallback(widgets, widgetsDispatch);
  const updateShortcuts = useUpdateShortcutsCallback(appSettingsDispatch);

  useRescanInitially(widgetsDispatch);
  useExitAppListener(appSettings, widgets);
  useUpdateSettingsListener(widgetsDispatch);

  return (
    <RadixTheme
      appearance={appSettings.theme}
      accentColor="indigo"
      grayColor="slate"
      css={{ height: "100vh" }}
    >
      <Toaster
        position="bottom-center"
        gap={6}
        toastOptions={{
          style: {
            color: "var(--gray-12)",
            borderColor: "var(--gray-6)",
            backgroundColor: "var(--gray-2)",
            padding: "var(--space-2) var(--space-4)",
          },
        }}
      />
      <ThemeToggler
        theme={appSettings.theme}
        appSettingsDispatch={appSettingsDispatch}
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
                  widgets={widgets}
                  widgetsDispatch={widgetsDispatch}
                  rescan={rescan}
                />
              </Box>
            </Tabs.Content>
            <Tabs.Content value="settings" asChild>
              <Box height="100%">
                <SettingsTab
                  appSettings={appSettings}
                  updateShortcuts={updateShortcuts}
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
};
