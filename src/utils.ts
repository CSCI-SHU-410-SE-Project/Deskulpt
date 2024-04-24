/**
 * Grab as much information as possible from the unknown error.
 *
 * A string error will be returned as is. An Error object will return its stack if it
 * exists, otherwise its message. An unknown error will return a generic message.
 *
 * @param err The unknown error, commonly from `catch`.
 * @returns The error information.
 */
export function grabErrorInfo(err: unknown) {
  if (typeof err === "string") {
    return err;
  }
  if (err instanceof Error) {
    return err.stack ?? err.message;
  }
  return "Unknown error caught that is neither a string nor an Error";
}

/**
 * Create a blob for widget APIs.
 *
 * This function fetches the template of widget APIs and replaces the placeholder with
 * the actual widget ID. The template essentially refers to raw APIs, converts them to
 * widget-specific APIs, and exports them. The blob is meant to be wrapped in a URL for
 * dynamic import using `createObjectURL`.
 *
 * @param widgetId The ID of the widget to create APIs for.
 * @returns The blob object wrapping the widget APIs.
 */
export async function createWidgetApisBlob(widgetId: string) {
  // The template is in the public directory, bundled from `tooling/apis`
  const response = await fetch("/.wrap-apis.js.txt");
  const template = await response.text();
  return new Blob([template.replace("__DESKULPT_WIDGET_ID__", widgetId)], {
    type: "application/javascript",
  });
}
