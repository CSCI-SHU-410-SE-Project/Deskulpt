/**
 * This file contains the types and interfaces that have backend counterparts.
 */

export type Result<T, E> = { Ok: T } | { Err: E };

export type IdMap<T> = Record<string, T>;

export type ToastKind = "success";

export interface ShowToastPayload {
  kind: ToastKind;
  message: string;
}

export type WidgetConfigMap = IdMap<Result<WidgetConfig, string>>;

export interface WidgetConfig {
  deskulptConf: DeskulptConf;
  externalDeps: Record<string, string>;
  directory: string;
}

export interface DeskulptConf {
  name: string;
  entry: string;
  ignore: boolean;
}

export type ThemeAppearance = "light" | "dark";

export interface Settings {
  themeAppearance: ThemeAppearance;
  toggleShortcut: string | null;
  widgetSettings: IdMap<WidgetSetting>;
}

export interface WidgetSetting {
  x: number;
  y: number;
  opacity: number;
}
