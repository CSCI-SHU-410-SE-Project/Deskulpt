import { create } from "zustand";
import { ShortcutsUpdate } from "../../bindings/types";
import { commands } from "../../core";
import { SwitchThemeEventAPI } from "../../bindings/events";

export const useAppSettingsStore = create(() => ({
  ...window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.app,
}));

export async function toggleTheme() {
  const theme = useAppSettingsStore.getState().theme;
  const newTheme = theme === "light" ? "dark" : "light";

  useAppSettingsStore.setState({ theme: newTheme });
  await SwitchThemeEventAPI.emitTo("canvas", newTheme);
}

export async function updateShortcut(update: ShortcutsUpdate) {
  await commands.updateSettings({
    updates: [{ field: "APP", value: { field: "SHORTCUTS", value: update } }],
  });

  useAppSettingsStore.setState((state) => ({
    shortcuts: { ...state.shortcuts, [update.field]: update.value },
  }));
}
