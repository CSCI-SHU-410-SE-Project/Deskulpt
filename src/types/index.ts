/**
 * This file contains definitions of all types and interfaces.
 */

import { ReactNode } from "react";

export * from "./backend";
export * from "./frontend";

/**
 * The user-defined widget interface.
 *
 * The entry file of each user-defined widget should export an object that fulfills this
 * interface as default.
 */
export interface Widget {
  /**
   * The function that defined the widget element to render.
   *
   * This function will be evaluated on widget load or refresh, where its output will
   * be wrapped within `React.StrictMode` to render on the desktop.
   *
   * @returns The React element to render.
   */
  render: () => ReactNode;
}
