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

    const newWidgets = Object.entries(configMap).map(([id, config]) => {
      const settings =
        widgets.find((widget) => widget.id === id)?.settings ??
        window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgets[id] ??
        DEFAULT_WIDGET_SETTINGS;
      return { id, config, settings };
    });

    await emitRenderToCanvas(
      newWidgets.map(({ id, settings }) => ({ id, settings })),
    );

    widgetsDispatch({
      type: WidgetsActionType.RESET_ALL,
      payload: { widgets: newWidgets },
    });

    return {
      numAdded: addedIds.length,
      numRemoved: removedIds.length,
      numUpdated: newWidgets.length - addedIds.length,
    };
  }, [widgets, widgetsDispatch]);
}
