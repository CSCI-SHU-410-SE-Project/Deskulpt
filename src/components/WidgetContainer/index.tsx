import { Box, Tooltip } from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import React from "react";
import Draggable from "react-draggable";
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
export default function WidgetContainer(props: {
  id: string;
  inner: React.ReactElement;
}) {
  const { id, inner } = props;

  return (
    <React.StrictMode>
      <Draggable>
        <Box
          sx={{
            px: 2,
            py: 1,
            m: 1,
            borderRadius: 1,
            border: "2px solid black",
            bgcolor: "rgba(0, 0, 0, 0.2)",
          }}
        >
          <Tooltip title={id} placement="left">
            <InfoIcon
              sx={{
                position: "absolute",
                top: 5,
                zIndex: 2000,
                fontSize: 15,
              }}
            />
          </Tooltip>
          <ErrorBoundary fallbackRender={fallbackRender}>{inner}</ErrorBoundary>
        </Box>
      </Draggable>
    </React.StrictMode>
  );
}

/**
 * The fallback component if the user widget fails to render.
 */
function fallbackRender(props: { error: unknown }) {
  const { error } = props;

  return (
    <ErrorDisplay
      title="Rendering error caught by the React error boundary"
      error={grabErrorInfo(error)}
    />
  );
}
