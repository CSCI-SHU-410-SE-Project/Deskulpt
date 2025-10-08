import { create } from "zustand";
import { ShortcutKey, commands, events } from "../../bindings";

export const useAppSettingsStore = create(() => ({
  ...window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.app,
}));

export async function toggleTheme() {
  const theme = useAppSettingsStore.getState().theme;
  const newTheme = theme === "light" ? "dark" : "light";

  useAppSettingsStore.setState({ theme: newTheme });
  await events.switchTheme.emitTo("canvas", newTheme);
}

export async function updateShortcut(key: ShortcutKey, shortcut?: string) {
  await commands.core.updateSettings({ shortcut: [key, shortcut ?? null] });
  useAppSettingsStore.setState((state) => ({
    shortcuts: { ...state.shortcuts, [key]: shortcut },
  }));
}
