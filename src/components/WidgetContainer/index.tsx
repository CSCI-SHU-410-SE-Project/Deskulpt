import { Box, Tooltip } from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import { ReactNode, useRef } from "react";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "../ErrorDisplay";
import { grabErrorInfo } from "../../utils";
import { WidgetInternal } from "../../types";

/**
 * The widget container component.
 *
 * It is a draggable and styled container that wraps the component to be displayed.
 */
export default function WidgetContainer(props: {
  id: string;
  internal: WidgetInternal;
  setInternal: (internal: WidgetInternal) => void;
  children: ReactNode;
}) {
  const { id, internal, setInternal, children } = props;
  const containerRef = useRef(null);

  /**
   * Update the container internal according to transform data.
   *
   * By default the `Draggable` component uses `transform` to move the container. This,
   * however, makes it impossible to obtain the actual position of the container, and
   * can cause mouse events to be misaligned with the actual position of the container.
   * The solution is to force zero `transform` and manually update the absolute position
   * of the container on dragging termination based on data reported by `Draggable`.
   */
  function updateContainerPos(_: DraggableEvent, data: DraggableData) {
    setInternal({ x: internal.x + data.x, y: internal.y + data.y });
  }

  return (
    <Draggable
      nodeRef={containerRef}
      position={{ x: 0, y: 0 }}
      onStop={updateContainerPos}
    >
      <Box
        ref={containerRef}
        sx={{
          padding: 1,
          borderRadius: 1,
          border: "2px solid black",
          backgroundColor: "rgba(0, 0, 0, 0.2)",
          position: "absolute",
          left: internal.x,
          top: internal.y,
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
          {children}
        </ErrorBoundary>
      </Box>
    </Draggable>
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
