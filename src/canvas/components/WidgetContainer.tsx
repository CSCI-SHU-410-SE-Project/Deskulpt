import { RefObject, memo, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { stringifyError } from "../../utils/stringifyError";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { updateWidgetSettings, useWidgetsStore } from "../hooks";
import { css } from "@emotion/react";

const styles = {
  wrapper: css({
    "&:hover": { ".handle": { opacity: 1 } },
  }),
  handle: css({
    cursor: "grab",
    opacity: 0,
    zIndex: 2,
    transition: "opacity 200ms ease-in-out",
  }),
  container: css({
    color: "var(--gray-12)",
    zIndex: 1,
  }),
};

interface WidgetContainerProps {
  id: string;
}

const WidgetContainer = memo(({ id }: WidgetContainerProps) => {
  const { Component, width, height, x, y, opacity } = useWidgetsStore(
    (state) => state.widgets[id],
  );
  const wrapperRef = useRef<HTMLDivElement>(null);

  const onStop = (_: DraggableEvent, data: DraggableData) => {
    updateWidgetSettings(id, { x: x + data.x, y: y + data.y }, true);
  };

  return (
    <Draggable
      // TODO: remove the `as` part which is workaround for React 19:
      // https://github.com/react-grid-layout/react-draggable/issues/768
      nodeRef={wrapperRef as RefObject<HTMLDivElement>}
      onStop={onStop}
      bounds="body"
      handle=".handle"
      position={{ x: 0, y: 0 }}
    >
      <Box
        ref={wrapperRef}
        overflow="hidden"
        position="absolute"
        css={styles.wrapper}
        style={{ left: x, top: y }}
      >
        <Box
          className="handle"
          position="absolute"
          top="1"
          right="1"
          css={styles.handle}
          asChild
        >
          <LuGripVertical size={20} />
        </Box>
        <Box
          position="relative"
          width={width ?? "300px"}
          height={height ?? "150px"}
          css={styles.container}
          style={{ opacity: opacity / 100 }}
        >
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
            <Component id={id} x={x} y={y} opacity={opacity} />
          </ErrorBoundary>
        </Box>
      </Box>
    </Draggable>
  );
});

export default WidgetContainer;
