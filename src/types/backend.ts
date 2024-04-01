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

/**
 * See `commands.rs` for its backend counterpart.
 *
 * @example
 * ```ts
 * function processCommandOut(out: CommandOut<string>) {
 *   if ("success" in out) {
 *     const content = out.success;
 *   } else {
 *     const error = out.failure;
 *   }
 * }
 * ```
 */
export type CommandOut<T> = { success: T } | { failure: string };
