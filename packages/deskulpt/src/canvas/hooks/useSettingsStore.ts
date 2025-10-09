import { create } from "zustand";
import { Settings } from "../../bindings";

export const useSettingsStore = create<Settings>(() => ({
  ...window.__DESKULPT_CANVAS_INTERNALS__.initialSettings,
}));
