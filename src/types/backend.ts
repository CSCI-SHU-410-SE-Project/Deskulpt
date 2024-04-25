/**
 * This file contains the types and interfaces that have backend counterparts
 */

/** See `config.rs` for its backend counterpart. */
export type WidgetCollection = Record<string, { Ok: WidgetConfig } | { Err: string }>;

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
