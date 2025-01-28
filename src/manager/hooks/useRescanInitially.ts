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

      const widgets = Object.entries(configMap).map(([id, config]) => {
        const settings =
          window.__DESKULPT__.initialSettings.widgets[id] ??
          DEFAULT_WIDGET_SETTINGS;
        return { id, config, settings };
      });

      await invokeEmitOnRenderReady({
        payload: widgets.map(({ id, settings }) => ({ id, settings })),
      });

      widgetsDispatch({
        type: WidgetsActionType.BATCH_UPDATE,
        payload: { widgets },
      });
    };

    helper().catch(console.error);
  }, [widgetsDispatch]);
}
