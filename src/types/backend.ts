/**
 * This file contains the types and interfaces that have backend counterparts.
 */

export type Result<T, E> = { Ok: T } | { Err: E };

export type WidgetCollection = Record<string, Result<WidgetConfig, string>>;

export interface WidgetConfig {
  name: string;
  entry: string;
  ignore: boolean;
  dependencies: Record<string, string>;
  directory: string;
}

export type Theme = "light" | "dark";

export interface Settings {
  theme: Theme;
  toggleShortcut?: string;
  widgetSettingsMap: Record<string, WidgetSettings>;
}

export interface WidgetSettings {
  x: number;
  y: number;
  opacity: number;
}
