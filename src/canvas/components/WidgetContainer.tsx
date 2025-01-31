import { RefObject, memo, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { UpdateSettingsCallback, WidgetState } from "../hooks";
import ErrorDisplay from "./ErrorDisplay";
import { stringifyError } from "../utils";
import { css } from "@emotion/react";

const styles = {
  container: css({
    color: "var(--gray-12)",
    backgroundColor: "var(--gray-surface)",
    borderRadius: "var(--radius-2)",
    boxShadow: "0 0 2px var(--gray-8)",
  }),
  dragger: css({
    position: "absolute",
    top: "var(--space-1)",
    right: 0,
    cursor: "grab",
    opacity: 0,
    zIndex: 9999,
    transition: "opacity 200ms ease-in-out",
    "&:hover": {
      opacity: 1,
    },
  }),
};

interface Props {
  id: string;
  widget: WidgetState;
  updateSettings: UpdateSettingsCallback;
}

export default memo(({ id, widget, updateSettings }: Props) => {
  const { Component, width, height, x, y, opacity } = widget;
  const containerRef = useRef<HTMLDivElement>(null);

  const updateContainerPos = (_: DraggableEvent, data: DraggableData) => {
    updateSettings(id, { x: x + data.x, y: y + data.y });
  };

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
        css={styles.container}
        style={{ opacity: `${opacity}%` }}
      >
        <LuGripVertical
          className="draggable-handle"
          size={20}
          css={styles.dragger}
        />
        <ErrorBoundary
          resetKeys={[id, widget]}
          fallbackRender={({ error }) => (
            <ErrorDisplay id={id} error={stringifyError(error)} />
          )}
        >
          <Component id={id} />
        </ErrorBoundary>
      </Box>
    </Draggable>
  );
});
