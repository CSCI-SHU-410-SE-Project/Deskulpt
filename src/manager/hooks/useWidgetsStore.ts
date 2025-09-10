import { create } from "zustand";
import { Widget, commands, events } from "../../bindings";
import { useSettingsStore } from "./useSettingsStore";

export const useWidgetsStore = create<Record<string, Widget>>(() => ({}));

export async function rescan(initial: boolean = false) {
  const configs = await commands.rescanWidgets();

  let widgetsArray;
  if (initial) {
    // Initial rescan assumes no widgets in the store
    widgetsArray = Object.entries(configs).map(([id, config]) => {
      return [id, config] as const;
    });
  } else {
    const currentWidgets = useWidgetsStore.getState();
    widgetsArray = Object.entries(configs).map(([id, config]) => {
      return [id, config] as const;
    });

    // Remove widgets that are no longer present
    const removedIds = Object.keys(currentWidgets).filter(
      (id) => !(id in configs),
    );
    if (removedIds.length > 0) {
      await events.removeWidgetsEvent.emitTo("canvas", removedIds);
    }
  }

  const widgetSettings = useSettingsStore.getState().widgets;
  const event = widgetsArray.map(([id]) => ({
    id,
    settings: widgetSettings[id],
  }));
  if (initial) {
    await commands.emitOnRenderReady({ event });
  } else {
    await events.renderWidgetsEvent.emitTo("canvas", event);
  }

  // Sort widgets by their directory name
  useWidgetsStore.setState(
    Object.fromEntries(
      widgetsArray.sort(([, a], [, b]) => a.dir.localeCompare(b.dir)),
    ),
  );

  return widgetsArray.length;
}

export function removeWidgets(ids: string[]) {
  useWidgetsStore.setState((state) =>
    Object.fromEntries(
      Object.entries(state).filter(([id]) => !ids.includes(id)),
    ),
  );
}
