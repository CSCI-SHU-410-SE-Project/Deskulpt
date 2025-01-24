import { Toaster } from "sonner";
import { Theme } from "@radix-ui/themes";
import {
  useBatchRemoveListener,
  useListenersReady,
  useRenderListener,
  useShowToastListener,
  useTheme,
  useUpdateSettingsCallback,
  useUpdateSettingsListener,
  useWidgets,
} from "./hooks";
import { WidgetContainer } from "./components";

export default () => {
  const ready = useListenersReady();

  const theme = useTheme(ready);
  const [widgets, widgetsDispatch] = useWidgets();
  const updateSettings = useUpdateSettingsCallback(widgetsDispatch);

  useRenderListener(widgets, widgetsDispatch, ready);
  useBatchRemoveListener(widgets, widgetsDispatch, ready);
  useShowToastListener(ready);
  useUpdateSettingsListener(widgetsDispatch, ready);

  return (
    <Theme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      hasBackground={false}
    >
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
      {Object.entries(widgets).map(([id, widget]) => (
        <WidgetContainer
          key={id}
          id={id}
          widget={widget}
          updateSettings={updateSettings}
        />
      ))}
    </Theme>
  );
};
