export enum Theme {
  LIGHT = "light",
  DARK = "dark",
}

export interface Shortcuts {
  canvasToggle: string | null;
}

export interface AppSettings {
  theme: Theme;
  shortcuts: Shortcuts;
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
