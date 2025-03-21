import { create } from "zustand";
import { Shortcuts, Theme } from "../../types";
import { commands, events } from "../../core";

export const useAppSettingsStore = create(() => ({
  ...window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.app,
}));

export async function toggleTheme() {
  const theme = useAppSettingsStore.getState().theme;
  const newTheme = theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT;

  useAppSettingsStore.setState({ theme: newTheme });
  await events.switchTheme.toCanvas({ theme: newTheme });
}

export async function updateShortcut(
  key: keyof Shortcuts,
  oldShortcut: string | null,
  newShortcut: string | null,
) {
  await commands.updateShortcut({ key, oldShortcut, newShortcut });
  useAppSettingsStore.setState((state) => ({
    shortcuts: { ...state.shortcuts, [key]: newShortcut },
  }));
}
