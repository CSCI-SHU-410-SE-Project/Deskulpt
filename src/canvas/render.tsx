import React from "react";
import ReactDOM from "react-dom/client";
import { RenderWidgetEventPayload, WidgetModule } from "../types";

/**
 * Render on the canvas based on the payload.
 *
 * This function has the side effect of updating `windows.__DESKULPT__.widgetStore`.
 *
 * @param canvas The canvas element to render on.
 * @param payload The payload of the "render-widget" event.
 */
export function renderWidget(canvas: HTMLElement, payload: RenderWidgetEventPayload) {
  const { widgetId, bundlerOutputPayload } = payload;

  if (bundlerOutputPayload.success) {
    // When the bundler succeeds, the payload message would be the bundled code
    const blob = new Blob([bundlerOutputPayload.message], {
      type: "application/javascript",
    });

    // Dynamically import the bundled code
    const url = URL.createObjectURL(blob);
    import(/* @vite-ignore */ url)
      .then((module: WidgetModule) => {
        const widget = module.default;

        // Get the DOM root for rendering
        const widgetDomRoot =
          widgetId in window.__DESKULPT__.widgetStore
            ? window.__DESKULPT__.widgetStore[widgetId].domRoot
            : createDomRoot(canvas, widgetId);

        // Render the widget under the corresponding DOM root
        const Component = widget.render();
        widgetDomRoot.render(
          <React.StrictMode>
            <Component />
          </React.StrictMode>,
        );

        // Store the widget details; always overwrite the previous details as long as
        // the rendering is successful
        window.__DESKULPT__.widgetStore[widgetId] = { domRoot: widgetDomRoot, widget };
      })
      .catch((err: Error) => {
        // FIXME: handle the error
        console.log({ err });
      });
  } else {
    // FIXME: handle the unsuccessful case
    console.log({ bundlerOutputPayload });
  }
}

function createDomRoot(canvas: HTMLElement, id: string): ReactDOM.Root {
  const widgetDiv = document.createElement("div");
  widgetDiv.id = id;
  const widgetDomRoot = ReactDOM.createRoot(widgetDiv);
  canvas.appendChild(widgetDiv);
  return widgetDomRoot;
}
