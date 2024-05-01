import { useState } from "react";
import { CanvasWidgetState, WidgetSetting } from "../types";
import WidgetContainer from "../components/WidgetContainer";
import { useRenderWidgetListener } from "../hooks/useRenderWidgetListener";
import { useRemoveWidgetsListener } from "../hooks/useRemoveWidgetsListener";
import { emitUpdateSettingToManager } from "../events";

export default function App() {
  const [canvasWidgetStates, setCanvasWidgetStates] = useState<
    Record<string, CanvasWidgetState>
  >({});

  useRenderWidgetListener(canvasWidgetStates, setCanvasWidgetStates);
  useRemoveWidgetsListener(canvasWidgetStates, setCanvasWidgetStates);

  /**
   * Update the setting of a particular widget.
   *
   * This function not only updates the setting in the canvas widget states, but also
   * notifies the manager to update the widget-specific setting as well.
   */
  async function setSettingForWidget(widgetId: string, setting: WidgetSetting) {
    await emitUpdateSettingToManager({ widgetId, setting });
    setCanvasWidgetStates((prev) => ({
      ...prev,
      [widgetId]: { ...prev[widgetId], setting },
    }));
  }

  return Object.entries(canvasWidgetStates).map(([widgetId, state]) => (
    <WidgetContainer
      key={widgetId}
      id={widgetId}
      setting={state.setting}
      setSetting={(setting) => setSettingForWidget(widgetId, setting)}
    >
      {state.display}
    </WidgetContainer>
  ));
}
