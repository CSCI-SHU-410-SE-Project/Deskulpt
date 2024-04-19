import { Box, Tooltip } from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import React from "react";
import Draggable from "react-draggable";

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
  const containerRef = React.useRef(null);
  const { id, inner } = props;

  return (
    <React.StrictMode>
      <Draggable nodeRef={containerRef}>
        <Box
          ref={containerRef}
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
          {inner}
        </Box>
      </Draggable>
    </React.StrictMode>
  );
}
