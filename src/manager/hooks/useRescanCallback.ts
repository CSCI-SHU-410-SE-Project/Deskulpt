import { useCallback } from "react";
import { invokeRescanWidgets } from "../../core/commands";
import { emitBatchRemoveToCanvas, emitRenderToCanvas } from "../../core/events";
import { WidgetsActionType, WidgetsDispatch, WidgetsState } from "./useWidgets";
import { DEFAULT_WIDGET_SETTINGS } from "../consts";

export type RescanCallback = () => Promise<{
  numAdded: number;
  numRemoved: number;
  numUpdated: number;
}>;

export function useRescanCallback(
  widgets: WidgetsState,
  widgetsDispatch: WidgetsDispatch,
) {
  return useCallback(async () => {
    const { configMap, addedIds, removedIds } = await invokeRescanWidgets();

    if (removedIds.length > 0) {
      await emitBatchRemoveToCanvas({ ids: removedIds });
    }

    const newWidgetsArray = Object.entries(configMap).map(([id, config]) => {
      const settings =
        widgets[id]?.settings ??
        window.__DESKULPT__.initialSettings.widgetSettingsMap[id] ??
        DEFAULT_WIDGET_SETTINGS;
      return [id, { config, settings }] as const;
    });

    await emitRenderToCanvas(
      newWidgetsArray.map(([id, { settings }]) => ({ id, settings })),
    );

    widgetsDispatch({
      type: WidgetsActionType.BATCH_UPDATE,
      payload: { widgets: Object.fromEntries(newWidgetsArray) },
    });

    return {
      numAdded: addedIds.length,
      numRemoved: removedIds.length,
      numUpdated: newWidgetsArray.length - addedIds.length,
    };
  }, []);
}
