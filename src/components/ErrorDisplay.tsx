import { Box } from "@mui/material";

/**
 * The error display component.
 *
 * It is used for displaying error messages in a formatted block. It requires the
 * following props:
 *
 * - `title`: A title that will be displayed above the error message.
 * - `error`: The error message to display. Whitespace characters are preserved.
 */
export default function ErrorDisplay(props: { title: string; error: string }) {
  const { title, error } = props;

  return (
    <Box>
      <Box
        sx={{
          fontWeight: "bold",
          color: "darkred",
        }}
      >
        {title}
      </Box>
      <Box
        sx={{
          whiteSpace: "pre-wrap",
          fontFamily: "monospace",
        }}
      >
        {error}
      </Box>
    </Box>
  );
}
