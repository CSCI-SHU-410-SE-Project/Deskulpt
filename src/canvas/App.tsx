import { useState } from "react";
import { CanvasWidgetState, WidgetSetting } from "../types";
import WidgetContainer from "../components/WidgetContainer";
import { useRenderWidgetListener } from "../hooks/useRenderWidgetListener";
import { useRemoveWidgetsListener } from "../hooks/useRemoveWidgetsListener";
import { emitUpdateSettingToManager } from "../events";
import { useShowToastListener } from "../hooks/useShowToastListener";

/**
 * The main component of the canvas window.
 */
export default function App() {
  const [canvasWidgetStates, setCanvasWidgetStates] = useState<
    Record<string, CanvasWidgetState>
  >({});

  const contextHolder = useShowToastListener();
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
    <>
      {contextHolder}
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
    </>
  );
}
