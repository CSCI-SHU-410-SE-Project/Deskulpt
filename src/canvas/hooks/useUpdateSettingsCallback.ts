import { useCallback } from "react";
import { WidgetSettings } from "../../types";
import { emitUpdateSettingsToManager } from "../../core/events";
import { WidgetsActionType, WidgetsDispatch } from "./useWidgets";

export type UpdateSettingsCallback = (
  id: string,
  settings: Partial<WidgetSettings>,
) => void;

export function useUpdateSettingsCallback(widgetsDispatch: WidgetsDispatch) {
  return useCallback(
    async (id: string, settings: Partial<WidgetSettings>) => {
      widgetsDispatch({
        type: WidgetsActionType.SET_SETTINGS,
        payload: { id, settings },
      });
      await emitUpdateSettingsToManager({ id, settings });
    },
    [widgetsDispatch],
  );
}
