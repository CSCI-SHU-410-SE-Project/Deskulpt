import { create } from "zustand";
import { Settings, WidgetConfigRegistry } from "../../bindings";

export const useSettings = create<Settings>(() => ({
  ...window.__DESKULPT_MANAGER_INTERNALS__.initialSettings,
}));

export const useWidgetConfigRegistry = create<WidgetConfigRegistry>(() => ({}));
