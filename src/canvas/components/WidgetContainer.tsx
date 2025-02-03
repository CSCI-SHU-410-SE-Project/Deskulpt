import { RefObject, memo, useCallback, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { stringifyError } from "../utils";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import {
  updateWidgetSettings,
  useWidgetsStore,
} from "../hooks/useWidgetsStore";
import { emitUpdateSettingsToManager } from "../../events";

interface WidgetContainerProps {
  id: string;
}

const WidgetContainer = memo(({ id }: WidgetContainerProps) => {
  const { Component, width, height, x, y, opacity } = useWidgetsStore(
    (state) => state.widgets[id],
  );
  const containerRef = useRef<HTMLDivElement>(null);

  const onStop = useCallback(
    (_: DraggableEvent, data: DraggableData) => {
      const pos = { x: x + data.x, y: y + data.y };
      updateWidgetSettings(id, pos);
      emitUpdateSettingsToManager({ id, settings: pos });
    },
    [id, x, y],
  );

  return (
    <Draggable
      // TODO: remove the `as` part which is workaround for React 19:
      // https://github.com/react-grid-layout/react-draggable/issues/768
      nodeRef={containerRef as RefObject<HTMLDivElement>}
      onStop={onStop}
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
          resetKeys={[Component]}
          fallbackRender={({ error }) => (
            <ErrorDisplay
              id={id}
              error="Error in the widget component [React error boundary]"
              message={stringifyError(error)}
            />
          )}
        >
          <Component id={id} />
        </ErrorBoundary>
      </Box>
    </Draggable>
  );
});

export default WidgetContainer;
