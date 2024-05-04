import { ReactNode, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { grabErrorInfo } from "../utils";
import { WidgetSetting } from "../types";
import { GripVertical } from "lucide-react";

/**
 * The widget container component that wraps around each user widget.
 */
export default function WidgetContainer(props: {
  id: string;
  setting: WidgetSetting;
  setSetting: (setting: WidgetSetting) => void;
  children: ReactNode;
  containerProps: { width: number | string; height: number | string };
}) {
  const { id, setting, setSetting, children, containerProps } = props;
  const containerRef = useRef(null);

  /**
   * Update the container position according to transform data.
   */
  function updateContainerPos(_: DraggableEvent, data: DraggableData) {
    setSetting({ x: setting.x + data.x, y: setting.y + data.y });
  }

  return (
    <Draggable
      nodeRef={containerRef}
      onStop={updateContainerPos}
      bounds="body"
      handle=".draggable-handle"
      position={{ x: 0, y: 0 }}
    >
      <div
        ref={containerRef}
        css={{
          overflow: "hidden",
          borderRadius: "5px",
          padding: "5px 10px",
          backgroundColor: "rgba(0, 0, 0, 0.7)",
          color: "#cccccc",
          boxShadow: "0 0 2px #888888",
          position: "absolute",
          left: setting.x,
          top: setting.y,
          width: containerProps.width,
          height: containerProps.height,
        }}
      >
        <GripVertical
          className="draggable-handle"
          css={{
            position: "absolute",
            top: "5px",
            right: "5px",
            width: "20px",
            height: "20px",
            cursor: "grab",
            opacity: "0",
            transition: "opacity 0.3s ease-in-out",
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
      </div>
    </Draggable>
  );
}
