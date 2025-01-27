import { useEffect } from "react";
import {
  invokeEmitOnRenderReady,
  invokeRescanWidgets,
} from "../../core/commands";
import { WidgetsActionType, WidgetsDispatch } from "./useWidgets";
import { DEFAULT_WIDGET_SETTINGS } from "../consts";

export function useRescanInitially(widgetsDispatch: WidgetsDispatch) {
  return useEffect(() => {
    const helper = async () => {
      const { configMap } = await invokeRescanWidgets();

      const widgetsArray = Object.entries(configMap).map(([id, config]) => {
        const settings =
          window.__DESKULPT__.initialSettings.widgets[id] ??
          DEFAULT_WIDGET_SETTINGS;
        return [id, { config, settings }] as const;
      });

      await invokeEmitOnRenderReady({
        payload: widgetsArray.map(([id, { settings }]) => ({ id, settings })),
      });

      widgetsDispatch({
        type: WidgetsActionType.BATCH_UPDATE,
        payload: { widgets: Object.fromEntries(widgetsArray) },
      });
    };

    helper().catch(console.error);
  }, [widgetsDispatch]);
}
