import { Toaster } from "sonner";
import { Theme } from "@radix-ui/themes";
import {
  useAppearance,
  useBatchRemoveListener,
  useRenderCallback,
  useRenderListener,
  useShowToastListener,
  useUpdateSettingsCallback,
  useUpdateSettingsListener,
  useWidgets,
} from "./hooks";
import { WidgetContainer } from "./components";

export default () => {
  // States
  const appearance = useAppearance();
  const [widgets, widgetsDispatch] = useWidgets();

  // Callbacks
  const render = useRenderCallback(widgets, widgetsDispatch);
  const updateSettings = useUpdateSettingsCallback(widgetsDispatch);

  // Listeners
  useBatchRemoveListener(widgets, widgetsDispatch);
  useRenderListener(render);
  useShowToastListener();
  useUpdateSettingsListener(widgetsDispatch);

  return (
    <Theme
      appearance={appearance}
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
