import { WidgetModule } from "../types";

/**
 * Validate a user widget module.
 *
 * If the module is invalid, the function returns an error message, and otherwise it
 * returns `null`. It ensures that the module provides a default export that contains
 * a `render` function.
 */
export function getWidgetModuleError(module: WidgetModule) {
  const widget = module.default;
  if (widget === undefined) {
    return "The widget must provide a default export.";
  }
  if (widget.render === undefined) {
    return "The default export of the widget must provide a `render` function.";
  }
  if (typeof widget.render !== "function") {
    return "The `render` key of the default export must be a function.";
  }
  return null;
}
