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
    <div
      css={{
        overflow: "auto",
        scrollbarWidth: "none",
        width: "100%",
        height: "100%",
      }}
    >
      <div css={{ fontWeight: "bold", color: "red" }}>{title}</div>
      <div css={{ whiteSpace: "pre-wrap", fontFamily: "monospace" }}>{error}</div>
    </div>
  );
}
