/**
 * This file contains the types and interfaces that have backend counterparts
 */

/** Default Serialization of the `Result` enum in Rust. */
export type Result<T, E> = { Ok: T } | { Err: E };

/** See `config.rs` for its backend counterpart. */
export type WidgetCollection = Record<string, Result<WidgetConfig, string>>;

/** See `config.rs` for its backend counterpart. */
export interface WidgetConfig {
  deskulptConf: DeskulptConf;
  externalDependencies: Record<string, string>;
  directory: string;
}

/** See `config.rs` for its backend counterpart. */
export interface DeskulptConf {
  name: string;
  entry: string;
  ignore: boolean;
}

/** See `states.rs` for its backend counterpart. */
export interface WidgetInternal {
  x: number;
  y: number;
}
