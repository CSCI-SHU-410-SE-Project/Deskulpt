import { Box } from "@mui/material";

export default function ErrorDisplay(props: { error: string }) {
  const { error } = props;

  return <Box component="pre">{error}</Box>;
}
