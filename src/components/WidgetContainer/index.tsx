import { Box, Tooltip } from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import React from "react";

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
      <Box
        sx={{
          px: 2,
          py: 1,
          m: 1,
          borderRadius: 1,
          bgcolor: "lightblue",
          position: "relative",
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
        {inner}
      </Box>
    </React.StrictMode>
  );
}
