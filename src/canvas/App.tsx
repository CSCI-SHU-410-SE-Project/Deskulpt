import { useState, useEffect } from "react";
import {
  RemoveWidgetsPayload,
  RenderWidgetPayload,
  WidgetCanvasState,
  WidgetInternal,
  WidgetModule,
} from "../types";
import { Event as TauriEvent, listen } from "@tauri-apps/api/event";
import { getWidgetModuleError } from "./utils";
import ErrorDisplay from "../components/ErrorDisplay";
import { grabErrorInfo } from "../utils";
import WidgetContainer from "../components/WidgetContainer";
import { invoke } from "@tauri-apps/api";

export default function App(props: {
  initialInternals: Record<string, WidgetInternal>;
}) {
  const { initialInternals } = props;
  const [widgetCanvasStates, setWidgetCanvasStates] = useState<
    Record<string, WidgetCanvasState>
  >({});

  /**
   * Get the internals of a widget.
   *
   * This will first look for the widget in the canvas states (the widgets are already
   * rendered and their internals might have changed), then look for the widget in the
   * initial internals (the widgets are not rendered but memoized, which we should
   * respect), and finally return the default internal for a brand new widget.
   */
  function getWidgetInternal(widgetId: string): WidgetInternal {
    if (widgetId in widgetCanvasStates) {
      return widgetCanvasStates[widgetId].internal;
    }
    if (widgetId in initialInternals) {
      return initialInternals[widgetId];
    }
    return { x: 0, y: 0 };
  }

  /**
   * Set the internals of a specific widget.
   */
  function setWidgetInternal(widgetId: string, internal: WidgetInternal) {
    setWidgetCanvasStates((prev) => ({
      ...prev,
      [widgetId]: {
        internal,
        display: prev[widgetId].display,
      },
    }));
  }

  useEffect(() => {
    // Listen to the "render-widget" event
    const unlistenRenderWidget = listen(
      "render-widget",
      (event: TauriEvent<RenderWidgetPayload>) => {
        const { widgetId, success, bundlerOutput } = event.payload;

        if (success) {
          // In this case the bundler output is the bundled code; we create an object URL
          // so that we can dynamically import the bundled code and obtain its export
          const blob = new Blob([bundlerOutput], { type: "application/javascript" });
          const url = URL.createObjectURL(blob);

          import(/* @vite-ignore */ url)
            .then((module: WidgetModule) => {
              // Early return before rendering if there are known errors in the widget
              const widgetModuleError = getWidgetModuleError(module);
              if (widgetModuleError !== null) {
                setWidgetCanvasStates((prev) => ({
                  ...prev,
                  [widgetId]: {
                    internal: getWidgetInternal(widgetId),
                    display: (
                      <ErrorDisplay
                        title={`Error in '${widgetId}': invalid widget module`}
                        error={widgetModuleError}
                      />
                    ),
                  },
                }));
                return;
              }

              // We have validated the module and can call the `render` function safely
              const widget = module.default;
              setWidgetCanvasStates((prev) => ({
                ...prev,
                [widgetId]: {
                  internal: getWidgetInternal(widgetId),
                  display: widget.render(),
                },
              }));
            })
            .catch((err) => {
              setWidgetCanvasStates((prev) => ({
                ...prev,
                [widgetId]: {
                  internal: getWidgetInternal(widgetId),
                  display: (
                    <ErrorDisplay
                      title={`Error in '${widgetId}': widget module failed to be imported`}
                      error={grabErrorInfo(err)}
                    />
                  ),
                },
              }));
            });
        } else {
          // In this case the bundler output is the error message
          setWidgetCanvasStates((prev) => ({
            ...prev,
            [widgetId]: {
              internal: getWidgetInternal(widgetId),
              display: (
                <ErrorDisplay
                  title={`Error in '${widgetId}': widget failed to be bundled`}
                  error={bundlerOutput}
                />
              ),
            },
          }));
        }
      },
    );

    // Listen to the "remove-widgets" event
    const unlistenRemoveWidgets = listen(
      "remove-widgets",
      (event: TauriEvent<RemoveWidgetsPayload>) => {
        const { removedIds } = event.payload;
        setWidgetCanvasStates((prev) =>
          Object.fromEntries(
            Object.entries(prev).filter(([widgetId]) => !removedIds.includes(widgetId)),
          ),
        );
      },
    );

    return () => {
      unlistenRenderWidget.then((f) => f()).catch(console.error);
      unlistenRemoveWidgets.then((f) => f()).catch(console.error);
    };
  }, []);

  useEffect(() => {
    // Listen to the "exit-app" event
    const unlistenExit = listen("exit-app", () => {
      const widgetInternals = Object.fromEntries(
        Object.entries(widgetCanvasStates).map(([widgetId, state]) => [
          widgetId,
          state.internal,
        ]),
      );
      invoke<null>("exit_app", { widgetInternals }).catch(console.error);
    });

    return () => {
      unlistenExit.then((f) => f()).catch(console.error);
    };
  }, [widgetCanvasStates]);

  return Object.entries(widgetCanvasStates).map(([widgetId, state]) => (
    <WidgetContainer
      key={widgetId}
      id={widgetId}
      internal={state.internal}
      setInternal={(interal: WidgetInternal) => setWidgetInternal(widgetId, interal)}
    >
      {state.display}
    </WidgetContainer>
  ));
}
