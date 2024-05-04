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
   * The function that defines the widget element to render.
   *
   * @returns The React element to render.
   */
  render: () => ReactNode;
  /**
   * The width of the widget.
   *
   * This can be a number in pixels, or a string that is accepted in CSS.
   */
  width: number | string;
  /**
   * The height of the widget.
   *
   * This can be a number in pixels, or a string that is accepted in CSS.
   */
  height: number | string;
}
