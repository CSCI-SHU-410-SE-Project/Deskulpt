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

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));

export function updateWidgetRender(
  id: string,
  Component: WidgetComponent,
  moduleBlobUrl: string,
  apisBlobUrl: string,
) {
  useWidgetsStore.setState((state) => ({
    ...state,
    [id]: {
      Component,
      moduleBlobUrl,
      apisBlobUrl,
    },
  }));
}

export function updateWidgetRenderError(
  id: string,
  error: string,
  message: string,
  apisBlobUrl: string,
) {
  useWidgetsStore.setState((state) => ({
    ...state,
    [id]: {
      Component: () => createElement(ErrorDisplay, { id, error, message }),
      moduleBlobUrl: undefined,
      apisBlobUrl,
    },
  }));
}

export function removeWidgets(ids: string[]) {
  const state = useWidgetsStore.getState();

  // Revoke object URLs for the widgets being removed
  ids.forEach((id) => {
    const widget = state[id];
    if (widget === undefined) {
      return;
    }
    URL.revokeObjectURL(widget.apisBlobUrl);
    widget.moduleBlobUrl && URL.revokeObjectURL(widget.moduleBlobUrl);
  });

  useWidgetsStore.setState((state) =>
    Object.fromEntries(
      Object.entries(state).filter(([id]) => !ids.includes(id)),
    ),
  );
}
