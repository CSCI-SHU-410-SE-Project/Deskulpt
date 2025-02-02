import { RefObject, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "../components/ErrorDisplay";
import { grabErrorInfo } from "../utils";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { WidgetState, updateWidgetSettings } from "../hooks/useWidgetsStore";
import { emitUpdateSettingsToManager } from "../../events";

interface WidgetContainerProps {
  /** ID of the widget. */
  id: string;
  /** The widget state. */
  widget: WidgetState;
}

/**
 * The widget container component that wraps around each user widget.
 *
 * It wraps the widget in a draggable container with a grip handle on the top right
 * corner on hover. It adds no padding within the container to allow users to have full
 * control over the appearance.
 *
 * If the child (i.e., the widget) throws a rendering error, it will be caught by the
 * error boundary and displayed with the {@link ErrorDisplay} component.
 */
const WidgetContainer = ({ id, widget }: WidgetContainerProps) => {
  const { Component, width, height, x, y, opacity } = widget;
  const containerRef = useRef<HTMLDivElement>(null);
  let retried = false;

  function updateContainerPos(_: DraggableEvent, data: DraggableData) {
    updateWidgetSettings(id, {
      x: x + data.x,
      y: y + data.y,
    });
    emitUpdateSettingsToManager({
      id,
      settings: { x: x + data.x, y: y + data.y, opacity },
    });
  }

  return (
    <Draggable
      // TODO: remove the `as` part which is workaround for React 19:
      // https://github.com/react-grid-layout/react-draggable/issues/768
      nodeRef={containerRef as RefObject<HTMLDivElement>}
      onStop={updateContainerPos}
      bounds="body"
      handle=".draggable-handle"
      position={{ x: 0, y: 0 }}
    >
      <Box
        ref={containerRef}
        overflow="hidden"
        position="absolute"
        left={`${x}px`}
        top={`${y}px`}
        width={width ?? "300px"}
        height={height ?? "150px"}
        css={{
          color: "var(--gray-12)",
          backgroundColor: "var(--gray-surface)",
          borderRadius: "var(--radius-2)",
          boxShadow: "0 0 2px var(--gray-8)",
          opacity: `${opacity}%`,
        }}
      >
        <LuGripVertical
          className="draggable-handle"
          size={20}
          css={{
            position: "absolute",
            top: "var(--space-1)",
            right: "var(--space-1)",
            cursor: "grab",
            opacity: "0",
            zIndex: 9999,
            transition: "opacity 200ms ease-in-out",
            "&:hover": {
              opacity: "1",
            },
          }}
        />
        <ErrorBoundary
          fallbackRender={({ error, resetErrorBoundary }) => {
            if (!retried) {
              // Reset the error boundary and retry the render once per re-render of the
              // widget, since otherwise even if the error in the children is fixed, the
              // error boundary will not refresh itself; note that the `retried` flag is
              // reset to false on each re-render and it is here to prevent infinite
              // loops of resetting error boundary and falling back
              resetErrorBoundary();
              retried = true;
            }
            return (
              <ErrorDisplay
                title={`Error in '${id}': potential issues with the \`render\` function)`}
                error={grabErrorInfo(error)}
              />
            );
          }}
        >
          <Component id={id} />
        </ErrorBoundary>
      </Box>
    </Draggable>
  );
};

export default WidgetContainer;
