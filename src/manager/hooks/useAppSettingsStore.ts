import { create } from "zustand";
import { Shortcuts, ShortcutsUpdate } from "../../bindings/types";
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

  let key: keyof Shortcuts;
  switch (update.field) {
    case "TOGGLE_CANVAS_IMODE":
      key = "toggleCanvasImode";
      break;
    case "OPEN_MANAGER":
      key = "openManager";
      break;
  }

  useAppSettingsStore.setState((state) => ({
    shortcuts: { ...state.shortcuts, [key]: update.value },
  }));
}
