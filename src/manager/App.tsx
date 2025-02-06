import { Box, Theme as RadixTheme, Tabs } from "@radix-ui/themes";
import { Toaster } from "sonner";
import {
  useAppSettingsStore,
  useExitAppListener,
  useInitialRescan,
  useUpdateSettingsListener,
} from "./hooks";
import About from "./components/About";
import Widgets from "./components/Widgets";
import Settings from "./components/Settings";
import ThemeToggler from "./components/ThemeToggler";

const tabs = [
  { value: "widgets", label: "Widgets", content: <Widgets /> },
  { value: "settings", label: "Settings", content: <Settings /> },
  { value: "about", label: "About", content: <About /> },
];

const App = () => {
  const theme = useAppSettingsStore((state) => state.theme);
  console.log("Rerendered!");

  useExitAppListener();
  useInitialRescan();
  useUpdateSettingsListener();

  return (
    <RadixTheme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      css={{ height: "100vh" }}
    >
      <Toaster
        position="bottom-center"
        theme={theme}
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
      <ThemeToggler theme={theme} />
      <Tabs.Root defaultValue="widgets" asChild>
        <Box height="100%" p="2">
          <Tabs.List>
            {tabs.map((tab) => (
              <Tabs.Trigger key={tab.value} value={tab.value}>
                {tab.label}
              </Tabs.Trigger>
            ))}
          </Tabs.List>
          <Box px="1" py="3" css={{ height: "calc(100% - 40px)" }}>
            {tabs.map((tab) => (
              <Tabs.Content key={tab.value} value={tab.value} asChild>
                <Box height="100%">{tab.content}</Box>
              </Tabs.Content>
            ))}
          </Box>
        </Box>
      </Tabs.Root>
    </RadixTheme>
  );
};

export default App;
