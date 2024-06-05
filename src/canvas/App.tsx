import { useState } from "react";
import { CanvasWidgetState } from "../types/frontend";
import { WidgetSetting, IdMap } from "../types/backend";
import { emitUpdateSettingToManager } from "../events";
import WidgetContainer from "./components/WidgetContainer";
import useRenderWidgetListener from "./hooks/useRenderWidgetListener";
import useRemoveWidgetsListener from "./hooks/useRemoveWidgetsListener";
import useShowToastListener from "./hooks/useShowToastListener";
import { Toaster } from "sonner";
import { Theme } from "@radix-ui/themes";
import useThemeAppearanceListener from "./hooks/useThemeAppearanceListener";

/**
 * The main component of the canvas window.
 */
export default function App() {
  const [canvasWidgetStates, setCanvasWidgetStates] = useState<
    IdMap<CanvasWidgetState>
  >({});
  const appearance = useThemeAppearanceListener();

  useShowToastListener();
  useRenderWidgetListener(canvasWidgetStates, setCanvasWidgetStates);
  useRemoveWidgetsListener(canvasWidgetStates, setCanvasWidgetStates);

  /**
   * Update the setting of a particular widget.
   *
   * This function not only updates the setting in the canvas widget states, but also
   * notifies the manager to update the widget-specific setting as well.
   */
  async function setSettingForWidget(widgetId: string, setting: WidgetSetting) {
    // This step must be done first, otherwise there will be a visible delay between
    // the transform change and the absolute position change, causing an undesirable
    // visual effect; TODO: figure out a way that do not use absolute position per
    // drag termination but still be able to keep the two-way control of position
    // between the manager and the canvas
    setCanvasWidgetStates((prev) => ({
      ...prev,
      [widgetId]: { ...prev[widgetId], setting },
    }));
    await emitUpdateSettingToManager({ widgetId, setting });
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
        ([widgetId, { display, width, height, setting }]) => (
          <WidgetContainer
            key={widgetId}
            id={widgetId}
            setting={setting}
            setSetting={(setting) => setSettingForWidget(widgetId, setting)}
            containerProps={{ width, height }}
          >
            {display}
          </WidgetContainer>
        ),
      )}
    </Theme>
  );
}
