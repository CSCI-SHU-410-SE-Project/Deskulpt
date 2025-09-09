import { create } from "zustand";
import { WidgetConfig, commands, events } from "../../bindings";

export const useWidgetsStore = create(() => ({
  configs: {} as Record<string, WidgetConfig>,
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
      await events.removeWidgetsEvent.emitTo("canvas", removedIds);
    }
  }

  const event = widgetsArray.map(([id, { settings }]) => ({ id, settings }));
  if (initial) {
    await commands.emitOnRenderReady({ event });
  } else {
    await events.renderWidgetsEvent.emitTo("canvas", event);
  }

  // Sort widgets by their directory name
  useWidgetsStore.setState({
    configs: Object.fromEntries(
      widgetsArray.sort(([, a], [, b]) =>
        a.config.dir.localeCompare(b.config.dir),
      ),
    ),
  });

  return widgetsArray.length;
}

export function removeWidgets(ids: string[]) {
  useWidgetsStore.setState((state) => ({
    configs: Object.fromEntries(
      Object.entries(state.configs).filter(([id]) => !ids.includes(id)),
    ),
  }));
}
