export enum Theme {
  LIGHT = "light",
  DARK = "dark",
}

export enum WidgetConfigType {
  VALID = "VALID",
  INVALID = "INVALID",
}

export type WidgetConfig =
  | {
      type: WidgetConfigType.VALID;
      content: {
        name: string;
        entry: string;
        dependencies: Record<string, string>;
      };
    }
  | { type: WidgetConfigType.INVALID; content: string };

export interface WidgetSettings {
  x: number;
  y: number;
  opacity: number;
}

export interface Settings {
  theme: Theme;
  toggleShortcut?: string;
  widgetSettingsMap: Record<string, WidgetSettings>;
}
