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

export type Appearance = "light" | "dark";

export interface Settings {
  appearance: Appearance;
  toggleShortcut?: string;
  widgetSettingsMap: Record<string, WidgetSettings>;
}

export interface WidgetSettings {
  x: number;
  y: number;
  opacity: number;
}
