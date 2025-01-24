import WidgetsTab from "./tabs/WidgetsTab";
import SettingsTab from "./tabs/SettingsTab";
import AboutTab from "./tabs/AboutTab";
import {
  useExitAppListener,
  useListenersReady,
  useRescanCallback,
  useTheme,
  useToggleShortcut,
  useUpdateSettingsListener,
  useWidgets,
  useWindowReadyListener,
} from "./hooks";
import { Box, Theme as RadixTheme, Tabs } from "@radix-ui/themes";
import { ThemeToggler } from "./components";
import { Toaster } from "sonner";

export default () => {
  const ready = useListenersReady();

  const [theme, toggleTheme] = useTheme();
  const [toggleShortcut, setToggleShortcut] = useToggleShortcut();
  const [widgets, widgetsDispatch] = useWidgets();
  const rescan = useRescanCallback(widgets, widgetsDispatch);

  useExitAppListener(toggleShortcut, theme, widgets, ready);
  useUpdateSettingsListener(widgetsDispatch, ready);
  useWindowReadyListener(rescan, ready);

  return (
    <RadixTheme
      appearance={theme}
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
      <ThemeToggler theme={theme} toggleTheme={toggleTheme} />
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
};
