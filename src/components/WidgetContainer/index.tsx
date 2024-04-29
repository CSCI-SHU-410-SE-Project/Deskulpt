import { Box, Tooltip } from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import { ReactNode, useRef, useState, StrictMode } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "../ErrorDisplay";
import { grabErrorInfo } from "../../utils";

/**
 * The widget container component.
 *
 * It is wrapped in `React.StrictMode` and should be rendered directly under the DOM
 * root. It requires the following `props`:
 *
 * - `id`: The ID of the widget to render.
 * - `inner`: The JSX element to render inside the container.
 */
export default function WidgetContainer(props: { id: string; inner: ReactNode }) {
  const { id, inner } = props;
  const containerRef = useRef(null);
  const [containerPos, setContainerPos] = useState({ x: 0, y: 0 });

  /**
   * Update the container position according to transform data.
   *
   * By default the `Draggable` component uses `transform` to move the container. This,
   * however, makes it impossible to obtain the actual position of the container, and
   * can cause mouse events to be misaligned with the actual position of the container.
   * The solution is to force zero `transform` and manually update the absolute position
   * of the container on dragging termination based on data reported by `Draggable`.
   */
  function updateContainerPos(_: DraggableEvent, data: DraggableData) {
    setContainerPos({ x: containerPos.x + data.x, y: containerPos.y + data.y });
  }

  return (
    <StrictMode>
      <Draggable
        nodeRef={containerRef}
        position={{ x: 0, y: 0 }}
        onStop={updateContainerPos}
      >
        <Box
          ref={containerRef}
          sx={{
            p: 1,
            borderRadius: 1,
            border: "2px solid black",
            bgcolor: "rgba(0, 0, 0, 0.2)",
            position: "absolute",
            left: containerPos.x,
            top: containerPos.y,
          }}
        >
          <Tooltip title={id} placement="left">
            <InfoIcon
              sx={{
                position: "absolute",
                top: 5,
                right: 5,
                zIndex: 2000,
                fontSize: 15,
              }}
            />
          </Tooltip>
          <ErrorBoundary fallbackRender={(props) => FallBack(id, props)}>
            {inner}
          </ErrorBoundary>
        </Box>
      </Draggable>
    </StrictMode>
  );
}

/**
 * The fallback component if the user widget fails to render.
 */
function FallBack(id: string, props: { error: unknown }) {
  const { error } = props;

  return (
    <ErrorDisplay
      title={`Error in '${id}': widget rendering failed (likely a problem with the React component returned by the \`render\` function)`}
      error={grabErrorInfo(error)}
    />
  );
}
