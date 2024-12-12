/**
 * This file contains the types and interfaces that have backend counterparts.
 */

/**
 * Default serialization of the `Result` enum in Rust.
 */
export type Result<T, E> = { Ok: T } | { Err: E };

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/utils/type.IdMap.html).
 */
export type IdMap<T> = Record<string, T>;

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/utils/enum.ToastKind.html).
 */
export type ToastKind = "success";

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/utils/struct.ShowToastPayload.html).
 */
export interface ShowToastPayload {
  kind: ToastKind;
  message: string;
}

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/config/type.WidgetConfigCollection.html).
 */
export type WidgetConfigCollection = IdMap<Result<WidgetConfig, string>>;

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/config/struct.WidgetConfig.html).
 */
export interface WidgetConfig {
  deskulptConf: DeskulptConf;
  externalDeps: Record<string, string>;
  directory: string;
}

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/config/struct.DeskulptConf.html).
 */
export interface DeskulptConf {
  name: string;
  entry: string;
  ignore: boolean;
}

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/settings/enum.ThemeAppearance.html).
 */
export type ThemeAppearance = "light" | "dark";

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/settings/struct.Settings.html).
 */
export interface Settings {
  themeAppearance: ThemeAppearance;
  toggleShortcut: string | null;
  widgetSettings: IdMap<WidgetSetting>;
}

/**
 * See [its backend counterpart](https://csci-shu-410-se-project.github.io/Deskulpt/rustdoc/deskulpt/settings/struct.WidgetSetting.html).
 */
export interface WidgetSetting {
  x: number;
  y: number;
  opacity: number;
}
