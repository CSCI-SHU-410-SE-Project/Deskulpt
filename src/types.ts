declare global {
  interface Window {
    readonly __DESKULPT__: {
      readonly apisWrapper: string;
      readonly initialSettings: DeepReadonly<Settings>;
    };
  }
}

export type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P];
};

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

export interface ReadonlyWidgetSettings {
  readonly x: number;
  readonly y: number;
  readonly opacity: number;
}

export interface ReadonlySettings {
  readonly theme: Theme;
  readonly toggleShortcut?: string;
  readonly widgetSettingsMap: {
    readonly [id: string]: ReadonlyWidgetSettings;
  };
}
