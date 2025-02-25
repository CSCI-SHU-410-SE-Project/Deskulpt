import { create } from "zustand";
import { WidgetConfig, WidgetSettings } from "../../types";
import { commands, events } from "../../core";

const DEFAULT_WIDGET_SETTINGS: WidgetSettings = { x: 0, y: 0, opacity: 100 };

interface WidgetState {
  config: WidgetConfig;
  settings: WidgetSettings;
}

export const useWidgetsStore = create(() => ({
  widgets: {} as Record<string, WidgetState>,
}));

export async function rescan(initial: boolean = false) {
  const configs = await commands.rescanWidgets();

  let widgetsArray;
  if (initial) {
    // Initial rescan assumes no widgets in the store
    widgetsArray = Object.entries(configs).map(([id, config]) => {
      const settings =
        window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgets[id] ??
        DEFAULT_WIDGET_SETTINGS;
      return [id, { config, settings }] as const;
    });
  } else {
    const currentWidgets = useWidgetsStore.getState().widgets;
    widgetsArray = Object.entries(configs).map(([id, config]) => {
      const settings =
        currentWidgets[id]?.settings ??
        window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgets[id] ??
        DEFAULT_WIDGET_SETTINGS;
      return [id, { config, settings }] as const;
    });

    // Remove widgets that are no longer present
    const removedIds = Object.keys(currentWidgets).filter(
      (id) => !(id in configs),
    );
    if (removedIds.length > 0) {
      await events.removeWidgets.toCanvas({ ids: removedIds });
    }
  }

  const payload = widgetsArray.map(([id, { settings }]) => ({ id, settings }));
  if (initial) {
    await commands.emitOnRenderReady({ payload });
  } else {
    await events.renderWidgets.toCanvas(payload);
  }

  // Sort widgets by their directory name
  useWidgetsStore.setState({
    widgets: Object.fromEntries(
      widgetsArray.sort(([, a], [, b]) =>
        a.config.content.dir.localeCompare(b.config.content.dir),
      ),
    ),
  });

  return widgetsArray.length;
}

export function updateWidgetSettings(
  id: string,
  settings: Partial<WidgetSettings>,
  emit: boolean = false,
) {
  useWidgetsStore.setState((state) => {
    if (id in state.widgets) {
      return {
        widgets: {
          ...state.widgets,
          [id]: {
            ...state.widgets[id],
            settings: { ...state.widgets[id].settings, ...settings },
          },
        },
      };
    }
    return {};
  });

  if (emit) {
    events.updateSettings.toCanvas({ id, settings }).catch(console.error);
  }
}

export function removeWidgets(ids: string[]) {
  useWidgetsStore.setState((state) => ({
    widgets: Object.fromEntries(
      Object.entries(state.widgets).filter(([id]) => !ids.includes(id)),
    ),
  }));
}
