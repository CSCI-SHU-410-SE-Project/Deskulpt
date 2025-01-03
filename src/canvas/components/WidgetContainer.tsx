import { PropsWithChildren, RefObject, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { grabErrorInfo } from "../utils";
import { WidgetSetting } from "../../types/backend";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { Widget } from "../../types/frontend";

export interface WidgetContainerProps {
  /** ID of the widget. */
  id: string;
  /** The setting of the widget. */
  setting: WidgetSetting;
  /** Callback function to update the setting of the specific widget. */
  setSetting: (setting: WidgetSetting) => void;
  /** Width of the widget container. */
  width: Widget["width"];
  /** Height of the widget container. */
  height: Widget["height"];
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
export default function WidgetContainer({
  id,
  setting,
  setSetting,
  width,
  height,
  children,
}: PropsWithChildren<WidgetContainerProps>) {
  const containerRef = useRef<HTMLDivElement>(null);
  let retried = false;

  function updateContainerPos(_: DraggableEvent, data: DraggableData) {
    setSetting({ ...setting, x: setting.x + data.x, y: setting.y + data.y });
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
        left={`${setting.x}px`}
        top={`${setting.y}px`}
        width={width}
        height={height}
        css={{
          color: "var(--gray-12)",
          backgroundColor: "var(--gray-surface)",
          borderRadius: "var(--radius-2)",
          boxShadow: "0 0 2px var(--gray-8)",
          opacity: `${setting.opacity}%`,
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
          {children}
        </ErrorBoundary>
      </Box>
    </Draggable>
  );
}
