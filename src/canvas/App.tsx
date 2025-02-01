import { useState } from "react";
import { CanvasWidgetState } from "../types/frontend";
import { WidgetSettings } from "../types/backend";
import { emitUpdateSettingsToManager } from "../events";
import WidgetContainer from "./components/WidgetContainer";
import useRenderWidgetListener from "./hooks/useRenderWidgetListener";
import useRemoveWidgetsListener from "./hooks/useRemoveWidgetsListener";
import useShowToastListener from "./hooks/useShowToastListener";
import { Toaster } from "sonner";
import { Theme as RadixTheme } from "@radix-ui/themes";
import useThemeListener from "./hooks/useThemeListener";

/**
 * The main component of the canvas window.
 */
export default function App() {
  const [canvasWidgetStates, setCanvasWidgetStates] = useState<
    Record<string, CanvasWidgetState>
  >({});
  const theme = useThemeListener();

  useShowToastListener();
  useRenderWidgetListener(canvasWidgetStates, setCanvasWidgetStates);
  useRemoveWidgetsListener(canvasWidgetStates, setCanvasWidgetStates);

  /**
   * Update the settings of a particular widget.
   *
   * This function not only updates the settings in the canvas widget states, but also
   * notifies the manager to update the widget-specific settings as well.
   */
  async function setSettingsForWidget(id: string, settings: WidgetSettings) {
    // This step must be done first, otherwise there will be a visible delay between
    // the transform change and the absolute position change, causing an undesirable
    // visual effect
    setCanvasWidgetStates((prev) => ({
      ...prev,
      [id]: { ...prev[id], settings },
    }));
    await emitUpdateSettingsToManager({ id, settings });
  }

  return (
    <RadixTheme
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
      {Object.entries(canvasWidgetStates).map(
        ([id, { display, width, height, settings }]) => (
          <WidgetContainer
            key={id}
            id={id}
            settings={settings}
            setSettings={(settings) => setSettingsForWidget(id, settings)}
            width={width}
            height={height}
          >
            {display}
          </WidgetContainer>
        ),
      )}
    </RadixTheme>
  );
}
