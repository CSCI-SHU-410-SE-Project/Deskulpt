import { ErrorBody, ErrorTitle, ScrollArea } from "./styled";

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
    <ScrollArea>
      <ErrorTitle>{title}</ErrorTitle>
      <ErrorBody>{error}</ErrorBody>
    </ScrollArea>
  );
}
