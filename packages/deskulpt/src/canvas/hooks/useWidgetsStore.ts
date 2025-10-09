import { create } from "zustand";
import { WidgetSettings } from "../../bindings";
import { FC, createElement } from "react";
import ErrorDisplay from "../components/ErrorDisplay";

interface WidgetProps extends WidgetSettings {
  id: string;
}

interface WidgetState {
  component: FC<WidgetProps>;
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));

export function updateWidgetRender(
  id: string,
  component: FC<WidgetProps>,
  moduleBlobUrl: string,
  apisBlobUrl: string,
) {
  useWidgetsStore.setState((state) => {
    return {
      ...state,
      [id]: {
        ...state[id],
        component,
        moduleBlobUrl,
        apisBlobUrl,
      },
    };
  });
}

export function updateWidgetRenderError(
  id: string,
  error: string,
  message: string,
  apisBlobUrl: string,
) {
  useWidgetsStore.setState((state) => {
    return {
      ...state,
      [id]: {
        ...state[id],
        component: () => createElement(ErrorDisplay, { id, error, message }),
        moduleBlobUrl: undefined,
        apisBlobUrl,
      },
    };
  });
}

export function updateWidgetSettings(
  id: string,
  settings: Partial<WidgetSettings>,
) {
  useWidgetsStore.setState((state) => ({
    ...state,
    [id]: { ...state[id], ...settings },
  }));
}

export function removeWidgets(ids: string[]) {
  const widgets = useWidgetsStore.getState();

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

  useWidgetsStore.setState((state) =>
    Object.fromEntries(
      Object.entries(state).filter(([id]) => !ids.includes(id)),
    ),
  );
}
