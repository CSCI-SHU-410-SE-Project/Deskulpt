import { memo, useCallback, useEffect, useRef, useState } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { Resizable, ResizeCallback } from "re-resizable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { stringifyError } from "../../utils/stringifyError";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { useSettingsStore, useWidgetsStore } from "../hooks";
import { css } from "@emotion/react";
import { commands } from "../../bindings";

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
  const draggableRef = useRef<HTMLDivElement>(null);

  const { component: Widget } = useWidgetsStore((state) => state[id]);
  const { opacity, ...settings } = useSettingsStore(
    (state) => state.widgets[id],
  );

  // Local state to avoid jittery movement during dragging and resizing
  const [x, setX] = useState(settings.x);
  const [y, setY] = useState(settings.y);
  const [width, setWidth] = useState(settings.width);
  const [height, setHeight] = useState(settings.height);

  useEffect(() => {
    setX(settings.x);
  }, [settings.x]);

  useEffect(() => {
    setY(settings.y);
  }, [settings.y]);

  useEffect(() => {
    setWidth(settings.width);
  }, [settings.width]);

  useEffect(() => {
    setHeight(settings.height);
  }, [settings.height]);

  const onDragStop = useCallback(
    (_: DraggableEvent, data: DraggableData) => {
      setX(data.x);
      setY(data.y);
      commands.core.updateSettings({
        widgets: { [id]: { x: data.x, y: data.y } },
      });
    },
    [id],
  );

  const onResizeStop: ResizeCallback = useCallback(
    (_, __, ___, delta) => {
      setWidth(width + delta.width);
      setHeight(height + delta.height);
      commands.core.updateSettings({
        widgets: {
          [id]: { width: width + delta.width, height: height + delta.height },
        },
      });
    },
    [id, width, height],
  );

  return (
    <Draggable
      nodeRef={draggableRef}
      position={{ x, y }}
      onStop={onDragStop}
      bounds="body"
      handle=".handle"
    >
      <Box
        ref={draggableRef}
        overflow="hidden"
        position="absolute"
        css={styles.wrapper}
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
        <Resizable
          size={{ width, height }}
          onResizeStop={onResizeStop}
          css={styles.container}
          style={{ opacity: opacity / 100 }}
        >
          <ErrorBoundary
            resetKeys={[Widget]}
            fallbackRender={({ error }) => (
              <ErrorDisplay
                id={id}
                error="Error in the widget component [React error boundary]"
                message={stringifyError(error)}
              />
            )}
          >
            <Widget
              id={id}
              x={x}
              y={y}
              width={width}
              height={height}
              opacity={opacity}
            />
          </ErrorBoundary>
        </Resizable>
      </Box>
    </Draggable>
  );
});

export default WidgetContainer;
