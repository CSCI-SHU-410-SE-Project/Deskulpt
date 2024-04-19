/**
 * This file contains the types and interfaces that have backend counterparts
 */

/** See `config.rs` for its backend counterpart. */
export interface WidgetConfig {
  deskulpt: DeskulptConf;
  node: PackageJson | null;
  directory: string;
}

/** See `config.rs` for its backend counterpart. */
export interface DeskulptConf {
  name: string;
  entry: string;
  ignore: boolean;
}

/** See `config.rs` for its backend counterpart. */
export interface PackageJson {
  dependencies: Record<string, string>;
}
