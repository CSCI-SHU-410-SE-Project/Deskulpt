import ReactDOM from "react-dom/client";
import ErrorDisplay from "../components/ErrorDisplay";
import WidgetContainer from "../components/WidgetContainer";
import { WidgetDOMRoot, WidgetRecord } from "../types";

/**
 * Grab as much information as possible from the unknown error.
 *
 * A string error will be returned as is. An Error object will return its stack if it
 * exists, otherwise its message. An unknown error will return a generic message.
 *
 * @param err The unknown error, commonly from `catch`.
 * @returns The error information.
 */
export function grabErrorInfo(err: unknown): string {
  if (typeof err === "string") {
    return err;
  }
  if (err instanceof Error) {
    return err.stack ?? err.message;
  }
  return `Unknown error caught that is neither a string nor an Error`;
}

/**
 * Handle a rendering error by trying to display error information in the DOM root.
 *
 * If the error information is rendered successfully, the widget record will be updated
 * correpondingly.
 *
 * If the error information is not rendered successfully, the DOM root will be unmounted
 * and removed to avoid leaving unused memory consumers on the canvas. Make sure that
 * the function is called as a final resolution.
 *
 * @param widgetId The ID of the widget for which the error occurred.
 * @param widgetDOMRoot The DOM root of the widget.
 * @param widgetRecords The records of widgets kept by the canvas.
 * @param title The title for the error. See the `ErrorDisplay` component.
 * @param error The error body. See the `ErrorDisplay` component.
 * @returns Whether the error information is rendered successfully.
 */
export function handleError(
  widgetId: string,
  widgetDOMRoot: WidgetDOMRoot,
  widgetRecords: Record<string, WidgetRecord>,
  title: string,
  error: string,
): boolean {
  try {
    widgetDOMRoot.react.render(
      <WidgetContainer
        id={widgetId}
        inner={<ErrorDisplay title={title} error={error} />}
      />,
    );
    widgetRecords[widgetId] = { root: widgetDOMRoot, error: true };
    return true;
  } catch (err) {
    // As the final resolution, if even the above fails we have no means to recover; we
    // unmount and remove the DOM root to prevent memory leaks
    console.error(err); // @Charlie-XIAO better error handling
    widgetDOMRoot.react.unmount();
    widgetDOMRoot.html.remove();
    return false;
  }
}

/**
 * Re-use or create a DOM root for rendering the specified widget.
 *
 * If the widget ID already exists in the records, this will re-use the recorded DOM
 * root. Otherwise, it will try to create a new one and append to the canvas. If this
 * step fails, the function will try to cleanup all intermediate resources created,
 * report the error to console, and return `null`.
 *
 * @param widgetId The ID of the widget to render.
 * @param widgetRecords The records of widgets kept by the canvas.
 * @param canvas The canvas element.
 * @returns The HTML and React DOM roots, or `null` if any error occurs.
 */
export function getDOMRoot(
  widgetId: string,
  widgetRecords: Record<string, WidgetRecord>,
  canvas: HTMLElement,
): WidgetDOMRoot | null {
  try {
    if (widgetId in widgetRecords) {
      // Re-use the existing DOM root
      return widgetRecords[widgetId].root;
    } else {
      // Create a new DOM root and append to the canvas
      const htmlDOMRoot = document.createElement("div");
      htmlDOMRoot.id = `deskulpt-widget--${widgetId}`;
      const reactDOMRoot = ReactDOM.createRoot(htmlDOMRoot);
      canvas.appendChild(htmlDOMRoot);
      return { html: htmlDOMRoot, react: reactDOMRoot };
    }
  } catch (err) {
    // The most likely reason for the error is that the react DOM root cannot be
    // successfully created; in that case we must avoid leaving an unused HTML div on
    // the canvas if it has already been created
    console.error(err); // @Charlie-XIAO better error handling
    const unusedDiv = document.getElementById(`deskulpt-widget--${widgetId}`);
    if (unusedDiv) {
      unusedDiv.remove();
    }
    return null;
  }
}
