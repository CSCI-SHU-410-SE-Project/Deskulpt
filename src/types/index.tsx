/**
 * This file contains definitions of all types and interfaces.
 */

import React from "react";
import { invoke } from "@tauri-apps/api";

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
 * The user-defined widget class.
 *
 * The entry file of each user-defined widget should export a class that extends the Widget class
 *
 * Note: Although class component is generally not recommended in React, we have to use this since
 * we need a way to associate widget api with widget data. Since a widget is just a html element
 * instead of a window, we need class to do the job.
 */
export class Widget extends React.Component {
  #widget_id = "";

  constructor(widget_id: string) {
    super({});
    this.#widget_id = widget_id;
  }

  get widget_id() {
    return this.#widget_id;
  }

  widget_api = {
    dummy: {
      shout: async (text: string) => {
        try {
          // const result = await window.__TAURI_INVOKE__(
          //   {cmd: `plugin:widget_api.dummy|shout_text`, text: text }
          // );
          const result = await invoke("plugin:widget_api.dummy|shout_text", {
            widgetId: this.#widget_id,
            text: text,
          });
          console.log(result);
          return result;
        } catch (error) {
          console.error(error);
          return error;
        }
      },
    },
    fs: {
      read_file: async (path: string) => {
        try {
          // const result = await window.__TAURI_INVOKE__(
          //   {cmd: `plugin:widget_api.fs|read_file`, path: path }
          // );
          const result = await invoke("plugin:widget_api.fs|read_file", {
            widgetId: this.#widget_id,
            path: path,
          });
          console.log(result);
          return result;
        } catch (error) {
          console.error(error);
        }
      },
    },
  };

  render(): React.JSX.Element {
    return <h1>Widget {this.#widget_id}</h1>;
  }
}
