import { create } from "zustand";
import { ShortcutKey, commands, events } from "../../bindings";

export const useAppSettingsStore = create(() => ({
  theme: window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.theme,
  shortcuts: window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.shortcuts,
}));

export async function toggleTheme() {
  const theme = useAppSettingsStore.getState().theme;
  const newTheme = theme === "light" ? "dark" : "light";

  useAppSettingsStore.setState({ theme: newTheme });
  await events.switchThemeEvent.emitTo("canvas", newTheme);
}

export async function updateShortcut(
  key: ShortcutKey,
  shortcut: string | null,
) {
  await commands.updateShortcut({ key, shortcut });
  useAppSettingsStore.setState((state) => ({
    shortcuts: { ...state.shortcuts, [key]: shortcut },
  }));
}
