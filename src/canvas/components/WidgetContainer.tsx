import { PropsWithChildren, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { grabErrorInfo } from "../../utils";
import { WidgetSetting } from "../../types/backend";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { Widget } from "../../types/frontend";

interface WidgetContainerProps {
  id: string;
  setting: WidgetSetting;
  setSetting: (setting: WidgetSetting) => void;
  containerProps: {
    width: Widget["width"];
    height: Widget["height"];
  };
}

/**
 * The widget container component that wraps around each user widget.
 */
export default function WidgetContainer({
  id,
  setting,
  setSetting,
  containerProps,
  children,
}: PropsWithChildren<WidgetContainerProps>) {
  const containerRef = useRef(null);

  /**
   * Update the container position according to transform data.
   */
  function updateContainerPos(_: DraggableEvent, data: DraggableData) {
    setSetting({ ...setting, x: setting.x + data.x, y: setting.y + data.y });
  }

  return (
    <Draggable
      nodeRef={containerRef}
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
        width={containerProps.width}
        height={containerProps.height}
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
          css={{
            position: "absolute",
            top: "5px",
            right: "5px",
            width: "20px",
            height: "20px",
            cursor: "grab",
            opacity: "0",
            transition: "opacity 200ms ease-in-out",
            "&:hover": {
              opacity: "1",
            },
          }}
        />
        <ErrorBoundary
          fallbackRender={({ error }) => (
            <ErrorDisplay
              title={`Error in '${id}': potential issues with the \`render\` function)`}
              error={grabErrorInfo(error)}
            />
          )}
        >
          {children}
        </ErrorBoundary>
      </Box>
    </Draggable>
  );
}
