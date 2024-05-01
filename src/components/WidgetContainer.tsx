import { Box, Tooltip } from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import DragHandleIcon from "@mui/icons-material/DragHandle";
import { ReactNode, useMemo, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { grabErrorInfo } from "../utils";
import { WidgetSetting } from "../types";

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
}) {
  const { id, setting, setSetting, children } = props;
  const containerRef = useRef(null);

  // Use an empty dependency array so that `useMemo` will evaluate only once, and the
  // resulting value would be the initial setting
  const initialSetting = useMemo(() => setting, []);

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
      handle=".draggable-handle"
    >
      <Box
        ref={containerRef}
        sx={{
          padding: 1,
          borderRadius: 1,
          border: "2px solid black",
          backgroundColor: "rgba(0, 0, 0, 0.2)",
          position: "absolute",
          left: initialSetting.x,
          top: initialSetting.y,
        }}
      >
        <Tooltip title={id} placement="left">
          <InfoIcon
            sx={{
              position: "absolute",
              top: 5,
              right: 25,
              zIndex: 2000,
              fontSize: 15,
            }}
          />
        </Tooltip>
        <DragHandleIcon
          className="draggable-handle"
          sx={{
            position: "absolute",
            top: 5,
            right: 5,
            zIndex: 2000,
            fontSize: 15,
          }}
        />
        <ErrorBoundary fallbackRender={({ error }) => FallBack(id, error)}>
          {children}
        </ErrorBoundary>
      </Box>
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
