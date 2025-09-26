import { create } from "zustand";
import { Settings, WidgetSettings } from "../../bindings";
import { FC } from "react";

interface WidgetProps extends WidgetSettings {
  id: string;
}

export interface WidgetModule {
  component: FC<WidgetProps>;
  width: string;
  height: string;
}

interface WidgetState extends WidgetModule {
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useSettings = create<Settings>(() => ({
  ...window.__DESKULPT_CANVAS_INTERNALS__.initialSettings,
}));

export const useWidgets = create<Record<string, WidgetState>>(() => ({}));
