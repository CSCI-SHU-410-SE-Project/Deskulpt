import { memo, useEffect, useRef, useState } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { stringifyError } from "../../utils/stringifyError";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { css } from "@emotion/react";
import { commands } from "../../bindings";
import { useSettings, useWidgets } from "../hooks/useStores";

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
  const wrapperRef = useRef<HTMLDivElement>(null);

  const settings = useSettings((state) => state.widgets[id]);
  const {
    component: Component,
    width,
    height,
  } = useWidgets((state) => state[id]!);

  const [x, setX] = useState(settings?.x ?? 0);
  const [y, setY] = useState(settings?.y ?? 0);

  useEffect(() => {
    setX(settings?.x ?? 0);
  }, [settings?.x]);

  useEffect(() => {
    setY(settings?.y ?? 0);
  }, [settings?.y]);

  const onStop = (_: DraggableEvent, data: DraggableData) => {
    setX(data.x);
    setY(data.y);
    commands.updateSettings({
      update: { widget: [id, { x: data.x, y: data.y }] },
    });
  };

  return (
    settings !== undefined && (
      <Draggable
        nodeRef={wrapperRef}
        onStop={onStop}
        bounds="body"
        handle=".handle"
        position={{ x, y }}
      >
        <Box
          ref={wrapperRef}
          overflow="hidden"
          position="absolute"
          width={width}
          height={height}
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
          <Box
            position="relative"
            width="100%"
            height="100%"
            css={styles.container}
            style={{ opacity: settings.opacity / 100 }}
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
              <Component id={id} x={x} y={y} opacity={settings.opacity} />
            </ErrorBoundary>
          </Box>
        </Box>
      </Draggable>
    )
  );
});

export default WidgetContainer;
