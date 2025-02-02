import { create } from "zustand";
import { WidgetSettings } from "../../types/backend";
import { FC, createElement } from "react";
import ErrorDisplay from "../components/ErrorDisplay";
import { grabErrorInfo } from "../utils";

export interface Widget {
  Component: FC<{ id: string }>;
  width?: string;
  height?: string;
}

export interface WidgetState extends Widget, WidgetSettings {
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create(() => ({
  widgets: {} as Record<string, WidgetState>,
}));

/**
 * Update rendering information of a widget.
 *
 * If the widget is in the store, rendering information will be updated, and the
 * settings will be ignored. Otherwise, the settings are required and a new
 * widget will be added to the store.
 */
export function updateWidgetRender(
  id: string,
  widget: Widget,
  moduleBlobUrl: string,
  apisBlobUrl: string,
  settings?: WidgetSettings,
) {
  useWidgetsStore.setState((state) => {
    if (id in state.widgets) {
      return {
        widgets: {
          ...state.widgets,
          [id]: {
            ...state.widgets[id],
            // Not using spread syntax because undefined properties in the
            // widget need to override their previous values as well
            Component: widget.Component,
            width: widget.width,
            height: widget.height,
            moduleBlobUrl,
          },
        },
      };
    }
    if (settings !== undefined) {
      return {
        widgets: {
          ...state.widgets,
          [id]: { ...widget, ...settings, apisBlobUrl, moduleBlobUrl },
        },
      };
    }
    return state;
  });
}

/**
 * Update rendering error of a widget.
 *
 * If the widget is in the store, its rendering information will be overridden
 * with the error and the settings will be ignored. Otherwise, the settings are
 * required and a new widget will be added to the store with the error.
 */
export function updateWidgetRenderError(
  id: string,
  error: unknown,
  apisBlobUrl: string,
  settings?: WidgetSettings,
) {
  useWidgetsStore.setState((state) => {
    if (id in state.widgets) {
      return {
        widgets: {
          ...state.widgets,
          [id]: {
            ...state.widgets[id],
            Component: () =>
              createElement(ErrorDisplay, {
                title: id,
                error: grabErrorInfo(error),
              }),
            width: undefined,
            height: undefined,
            moduleBlobUrl: undefined,
          },
        },
      };
    }
    if (settings !== undefined) {
      return {
        widgets: {
          ...state.widgets,
          [id]: {
            ...settings,
            Component: () =>
              createElement(ErrorDisplay, {
                title: id,
                error: grabErrorInfo(error),
              }),
            apisBlobUrl,
          },
        },
      };
    }
    return state;
  });
}

/**
 * Update (partial) settings of a widget.
 */
export function updateWidgetSettings(
  id: string,
  settings: Partial<WidgetSettings>,
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
}

/**
 * Remove a batch of widgets from the store.
 */
export function removeWidgets(ids: string[]) {
  useWidgetsStore.setState((state) => ({
    widgets: Object.fromEntries(
      Object.entries(state.widgets).filter(([id]) => !ids.includes(id)),
    ),
  }));
}
