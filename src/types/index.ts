/**
 * This file contains definitions of all types and interfaces.
 */

import React from "react";

export * from "./backend";
export * from "./frontend";

declare global {
  interface Window {
    /**
     * Global Deskulpt information that intends to be accessible by the widgets.
     */
    __DESKULPT__: {
      /**
       * Default Deskulpt dependencies.
       *
       * These are the packages that are always available to the widgets. Widget
       * developers should access these packages through this object instead of
       * importing them directly.
       *
       * @example
       * ```tsx
       * const React = window.__DESKULPT__.defaultDeps.React;
       * function Counter() {
       *   const [count, setCount] = React.useState(0);
       *   return <h1>{count}</h1>;
       * }
       * ```
       */
      defaultDeps: {
        React: typeof React;
      };
    };
  }
}

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
  render: () => React.ReactElement;
}
