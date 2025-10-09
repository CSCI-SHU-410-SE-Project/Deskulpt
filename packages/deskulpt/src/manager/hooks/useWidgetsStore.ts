import { create } from "zustand";
import { WidgetConfig, commands, events } from "../../bindings";

export const useWidgetsStore = create<Record<string, WidgetConfig>>(() => ({}));

export async function rescan(initial: boolean = false) {
  const configs = await commands.core.rescanWidgets();

  let widgetsArray;
  if (initial) {
    // Initial rescan assumes no widgets in the store
    widgetsArray = Object.entries(configs).map(([id, config]) => {
      return [id, config] as const;
    });
  } else {
    const currentWidgets = useWidgetsStore.getState().widgets;
    widgetsArray = Object.entries(configs).map(([id, config]) => {
      return [id, config] as const;
    });

    // Remove widgets that are no longer present
    const removedIds = Object.keys(currentWidgets).filter(
      (id) => !(id in configs),
    );
    if (removedIds.length > 0) {
      await events.removeWidgets.emitTo("canvas", removedIds);
    }
  }

  const event = widgetsArray.map(([id]) => id);
  if (initial) {
    await commands.core.emitOnRenderReady(event);
  } else {
    await events.renderWidgets.emitTo("canvas", event);
  }

  // Sort widgets by their directory name
  useWidgetsStore.setState(
    Object.fromEntries(widgetsArray.toSorted(([a], [b]) => a.localeCompare(b))),
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
