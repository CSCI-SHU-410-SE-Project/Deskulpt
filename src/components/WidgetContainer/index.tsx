import { Box, Tooltip } from "@mui/material";
import InfoIcon from "@mui/icons-material/Info";
import React from "react";

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
            fontSize="small"
            sx={{ position: "absolute", top: 5, right: 5, zIndex: 2000 }}
          />
        </Tooltip>
        {inner}
      </Box>
    </React.StrictMode>
  );
}
