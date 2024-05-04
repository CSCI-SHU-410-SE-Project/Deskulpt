/**
 * The error display component for user widget errors.
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
