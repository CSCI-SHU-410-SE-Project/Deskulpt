import { create } from "zustand";
import { WidgetConfig, WidgetSettings } from "../../types";
import { commands, events } from "../../core";

const DEFAULT_WIDGET_SETTINGS: WidgetSettings = { x: 0, y: 0, opacity: 100 };

interface WidgetState {
  id: string;
  config: WidgetConfig;
  settings: WidgetSettings;
}

export const useWidgetsStore = create(() => ({
  widgets: [] as WidgetState[],
}));

export async function rescan(initial: boolean = false) {
  const configs = await commands.rescanWidgets();

  let widgets: WidgetState[];
  if (initial) {
    // Initial rescan assumes no widgets in the store
    widgets = Object.entries(configs).map(([id, config]) => {
      const settings =
        window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgets[id] ??
        DEFAULT_WIDGET_SETTINGS;
      return { id, config, settings };
    });
  } else {
    const currentWidgets = useWidgetsStore.getState().widgets;
    widgets = Object.entries(configs).map(([id, config]) => {
      const settings =
        currentWidgets.find((widget) => widget.id === id)?.settings ??
        window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.widgets[id] ??
        DEFAULT_WIDGET_SETTINGS;
      return { id, config, settings };
    });

    // Remove widgets that are no longer present
    const removedIds = currentWidgets
      .map(({ id }) => id)
      .filter((id) => !(id in configs));
    if (removedIds.length > 0) {
      await events.removeWidgets.toCanvas({ ids: removedIds });
    }
  }

  const payload = widgets.map(({ id, settings }) => ({ id, settings }));
  if (initial) {
    await commands.emitOnRenderReady({ payload });
  } else {
    await events.renderWidgets.toCanvas(payload);
  }

  // Sort widgets by their directory name
  useWidgetsStore.setState({
    widgets: widgets.sort((a, b) =>
      a.config.content.dir.localeCompare(b.config.content.dir),
    ),
  });

  return widgets.length;
}

export function updateWidgetSettings(
  id: string,
  settings: Partial<WidgetSettings>,
) {
  useWidgetsStore.setState((state) => {
    const index = state.widgets.findIndex((widget) => widget.id === id);
    if (index === -1) return {};

    const widgets = [...state.widgets];
    widgets[index] = {
      ...state.widgets[index],
      settings: { ...state.widgets[index].settings, ...settings },
    };
    return { widgets };
  });
}

export function removeWidgets(ids: string[]) {
  useWidgetsStore.setState((state) => ({
    widgets: state.widgets.filter(({ id }) => !ids.includes(id)),
  }));
}
