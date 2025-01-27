import { Toaster } from "sonner";
import { Box, Theme as RadixTheme } from "@radix-ui/themes";
import {
  useBatchRemoveListener,
  useRender,
  useShowToastListener,
  useTheme,
  useUpdateSettingsCallback,
  useUpdateSettingsListener,
  useWidgets,
} from "./hooks";
import { RenderingScreen, WidgetContainer } from "./components";

export default () => {
  const theme = useTheme();
  const [widgets, widgetsDispatch] = useWidgets();
  const isRendering = useRender(widgets, widgetsDispatch);
  const updateSettings = useUpdateSettingsCallback(widgetsDispatch);

  useBatchRemoveListener(widgets, widgetsDispatch);
  useShowToastListener();
  useUpdateSettingsListener(widgetsDispatch);

  return (
    <RadixTheme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      hasBackground={false}
      css={{ height: "100vh" }}
    >
      {isRendering && <RenderingScreen />}
      <Toaster
        position="bottom-right"
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
      <Box>
        {Object.entries(widgets).map(([id, widget]) => (
          <WidgetContainer
            key={id}
            id={id}
            widget={widget}
            updateSettings={updateSettings}
          />
        ))}
      </Box>
    </RadixTheme>
  );
};
