import { useState } from "react";
import { CanvasWidgetState } from "@/types/frontend";
import { WidgetSettings } from "@/types/backend";
import { emitUpdateSettingsToManager } from "@/events";
import WidgetContainer from "@/canvas/components/WidgetContainer";
import useRenderWidgetListener from "@/canvas/hooks/useRenderWidgetListener";
import useRemoveWidgetsListener from "@/canvas/hooks/useRemoveWidgetsListener";
import useShowToastListener from "@/canvas/hooks/useShowToastListener";
import { Toaster } from "sonner";
import { Theme } from "@radix-ui/themes";
import useAppearanceListener from "@/canvas/hooks/useAppearanceListener";

/**
 * The main component of the canvas window.
 */
export default function App() {
  const [canvasWidgetStates, setCanvasWidgetStates] = useState<
    Record<string, CanvasWidgetState>
  >({});
  const appearance = useAppearanceListener();

  useShowToastListener();
  useRenderWidgetListener(canvasWidgetStates, setCanvasWidgetStates);
  useRemoveWidgetsListener(canvasWidgetStates, setCanvasWidgetStates);

  /**
   * Update the settings of a particular widget.
   *
   * This function not only updates the settings in the canvas widget states, but also
   * notifies the manager to update the widget-specific settings as well.
   */
  async function setSettingsForWidget(
    widgetId: string,
    settings: WidgetSettings,
  ) {
    // This step must be done first, otherwise there will be a visible delay between
    // the transform change and the absolute position change, causing an undesirable
    // visual effect
    setCanvasWidgetStates((prev) => ({
      ...prev,
      [widgetId]: { ...prev[widgetId], settings },
    }));
    await emitUpdateSettingsToManager({ widgetId, settings });
  }

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
      {Object.entries(canvasWidgetStates).map(
        ([widgetId, { display, width, height, settings }]) => (
          <WidgetContainer
            key={widgetId}
            id={widgetId}
            settings={settings}
            setSettings={(settings) => setSettingsForWidget(widgetId, settings)}
            width={width}
            height={height}
          >
            {display}
          </WidgetContainer>
        ),
      )}
    </Theme>
  );
}
