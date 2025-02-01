/**
 * This file contains the types and interfaces that have backend counterparts.
 */

export type ShowToastPayload = { success: string } | { error: string };

export enum WidgetConfigType {
  VALID = "VALID",
  INVALID = "INVALID",
}

export type WidgetConfig =
  | {
      type: WidgetConfigType.VALID;
      content: {
        dir: string;
        name: string;
        entry: string;
        dependencies: Record<string, string>;
      };
    }
  | {
      type: WidgetConfigType.INVALID;
      content: {
        dir: string;
        error: string;
      };
    };

export enum Theme {
  LIGHT = "light",
  DARK = "dark",
}

export interface Shortcuts {
  toggleCanvas: string | null;
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
