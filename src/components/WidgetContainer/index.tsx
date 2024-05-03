import { ReactNode, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "../ErrorDisplay";
import { grabErrorInfo } from "../../utils";
import { WidgetSetting } from "../../types";
import { DragHandle, Wrapper } from "./styled";

/**
 * The widget container component.
 *
 * It is a draggable and styled container that wraps the component to be displayed.
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
      <Wrapper
        ref={containerRef}
        style={{
          position: "absolute",
          left: setting.x,
          top: setting.y,
          width: containerProps.width,
          height: containerProps.height,
        }}
      >
        <DragHandle className="draggable-handle" />
        <ErrorBoundary fallbackRender={({ error }) => FallBack(id, error)}>
          {children}
        </ErrorBoundary>
      </Wrapper>
    </Draggable>
  );
}

/**
 * The fallback component of the error boundary.
 */
function FallBack(id: string, error: unknown) {
  return (
    <ErrorDisplay
      title={`Error in '${id}': potential issues with the \`render\` function)`}
      error={grabErrorInfo(error)}
    />
  );
}
