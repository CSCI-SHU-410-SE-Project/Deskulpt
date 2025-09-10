import { create } from "zustand";
import { WidgetSettings } from "../../bindings";
import { FC, createElement } from "react";
import ErrorDisplay from "../components/ErrorDisplay";

interface WidgetProps extends WidgetSettings {
  id: string;
}

type WidgetComponent = FC<WidgetProps>;

interface WidgetState {
  Component: WidgetComponent;
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create(() => ({
  widgets: {} as Record<string, WidgetState>,
}));

export function updateWidgetRender(
  id: string,
  Component: WidgetComponent,
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
            // Not using spread syntax because undefined properties in the
            // widget need to override their previous values as well
            Component,
            moduleBlobUrl,
          },
        },
      };
    }

    // Settings are required if the widget is newly added
    if (settings !== undefined) {
      return {
        widgets: {
          ...state.widgets,
          [id]: { ...settings, Component, apisBlobUrl, moduleBlobUrl },
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
            Component: () =>
              createElement(ErrorDisplay, { id, error, message }),
            width: undefined,
            height: undefined,
            moduleBlobUrl: undefined,
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
            Component: () =>
              createElement(ErrorDisplay, { id, error, message }),
            apisBlobUrl,
            width: undefined,
            height: undefined,
            moduleBlobUrl: undefined,
          },
        },
      };
    }

    return state;
  });
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
    widget.moduleBlobUrl && URL.revokeObjectURL(widget.moduleBlobUrl);
  });

  useWidgetsStore.setState((state) => ({
    widgets: Object.fromEntries(
      Object.entries(state.widgets).filter(([id]) => !ids.includes(id)),
    ),
  }));
}
