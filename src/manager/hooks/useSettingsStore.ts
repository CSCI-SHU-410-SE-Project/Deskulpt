import { create } from "zustand";
import { Settings } from "../../bindings";

export const useSettingsStore = create<{ settings: Settings }>(() => ({
  settings: window.__DESKULPT_MANAGER_INTERNALS__.initialSettings,
}));
