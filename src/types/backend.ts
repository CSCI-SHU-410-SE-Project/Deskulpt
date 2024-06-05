/**
 * This file contains the types and interfaces that have backend counterparts.
 */

/** Default serialization of the `Result` enum in Rust. */
export type Result<T, E> = { Ok: T } | { Err: E };

/** See `utils.rs` for its backend counterpart. */
export type IdMap<T> = Record<string, T>;

/** See `utils.rs` for its backend counterpart. */
export type ToastKind = "success";

/** See `utils.rs` for its backend counterpart. */
export interface ShowToastPayload {
  kind: ToastKind;
  message: string;
}

/** See `config.rs` for its backend counterpart. */
export type WidgetConfigCollection = IdMap<Result<WidgetConfig, string>>;

/** See `config.rs` for its backend counterpart. */
export interface WidgetConfig {
  deskulptConf: DeskulptConf;
  externalDeps: Record<string, string>;
  directory: string;
}

/** See `config.rs` for its backend counterpart. */
export interface DeskulptConf {
  name: string;
  entry: string;
  ignore: boolean;
}

/** See `settings.rs` for its backend counterpart. */
export type ThemeAppearance = "light" | "dark";

/** See `settings.rs` for its backend counterpart. */
export interface Settings {
  themeAppearance: ThemeAppearance;
  toggleShortcut: string | null;
  widgetSettings: IdMap<WidgetSetting>;
}

/** See `settings.rs` for its backend counterpart. */
export interface WidgetSetting {
  x: number;
  y: number;
  opacity: number;
}
