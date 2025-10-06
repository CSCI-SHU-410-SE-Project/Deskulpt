import { create } from "zustand";
import { WidgetSettings, events } from "../../bindings";
import { FC, createElement } from "react";
import ErrorDisplay from "../components/ErrorDisplay";

interface WidgetProps extends WidgetSettings {
  id: string;
}

interface WidgetState extends WidgetSettings {
  component: FC<WidgetProps>;
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create(() => ({
  widgets: {} as Record<string, WidgetState>,
}));

export function updateWidgetRender(
  id: string,
  component: FC<WidgetProps>,
  moduleBlobUrl: string,
  apisBlobUrl: string,
  settings?: WidgetSettings,
) {
  useWidgetsStore.setState((state) => {
    // Settings are ignored if the widget is already in the store
    if (id in state.widgets) {
      return {
        widgets: {
          ...state.widgets,
          [id]: {
            ...state.widgets[id],
            component,
            moduleBlobUrl,
            apisBlobUrl,
          },
        },
      };
    }

    // Settings are required if the widget is newly added
    if (settings !== undefined) {
      return {
        widgets: {
          ...state.widgets,
          [id]: { ...settings, component, moduleBlobUrl, apisBlobUrl },
        },
      };
    }

    return state;
  });
}

export function updateWidgetRenderError(
  id: string,
  error: string,
  message: string,
  apisBlobUrl: string,
  settings?: WidgetSettings,
) {
  useWidgetsStore.setState((state) => {
    // Settings are ignored if the widget is already in the store
    if (id in state.widgets) {
      return {
        widgets: {
          ...state.widgets,
          [id]: {
            ...state.widgets[id],
            component: () =>
              createElement(ErrorDisplay, { id, error, message }),
            moduleBlobUrl: undefined,
            apisBlobUrl,
          },
        },
      };
    }

    // Settings are required if the widget is newly added
    if (settings !== undefined) {
      return {
        widgets: {
          ...state.widgets,
          [id]: {
            ...settings,
            component: () =>
              createElement(ErrorDisplay, { id, error, message }),
            moduleBlobUrl: undefined,
            apisBlobUrl,
          },
        },
      };
    }

    return state;
  });
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
          [id]: { ...state.widgets[id], ...settings },
        },
      };
    }
    return state;
  });

  if (emit) {
    events.updateSettings
      .emitTo("manager", { id, ...settings })
      .catch(console.error);
  }
}

export function removeWidgets(ids: string[]) {
  const widgets = useWidgetsStore.getState().widgets;

  // Revoke object URLs for the widgets being removed
  ids.forEach((id) => {
    const widget = widgets[id];
    if (widget === undefined) {
      return; // This should not happen but let us be safe
    }
    URL.revokeObjectURL(widget.apisBlobUrl);
    if (widget.moduleBlobUrl !== undefined) {
      URL.revokeObjectURL(widget.moduleBlobUrl);
    }
  });

  useWidgetsStore.setState((state) => ({
    widgets: Object.fromEntries(
      Object.entries(state.widgets).filter(([id]) => !ids.includes(id)),
    ),
  }));
}
