/**
 * This file contains the types and interfaces that have backend counterparts.
 */

export type Result<T, E> = { Ok: T } | { Err: E };

export type ShowToastPayload = { success: string } | { error: string };

export type WidgetCollection = Record<string, Result<WidgetConfig, string>>;

export interface WidgetConfig {
  name: string;
  entry: string;
  ignore: boolean;
  dependencies: Record<string, string>;
  directory: string;
}

export type ThemeAppearance = "light" | "dark";

export interface GlobalSetting {
  themeAppearance: ThemeAppearance;
  toggleShortcut: string | null;
  widgetSettings: Record<string, WidgetSetting>;
}

export interface WidgetSetting {
  x: number;
  y: number;
  opacity: number;
}
