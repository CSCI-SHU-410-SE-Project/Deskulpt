export enum Theme {
  LIGHT = "light",
  DARK = "dark",
}

export enum ShortcutKey {
  TOGGLE_CANVAS = "TOGGLE_CANVAS",
  OPEN_MANAGER = "OPEN_MANAGER",
}

export interface AppSettings {
  theme: Theme;
  shortcuts: Partial<Record<ShortcutKey, string>>;
}

export interface WidgetSettings {
  x: number;
  y: number;
  opacity: number;
}

export interface Settings {
  app: AppSettings;
  widgets: Record<string, WidgetSettings>;
}
