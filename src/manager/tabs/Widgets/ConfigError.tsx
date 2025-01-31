import { css } from "@emotion/react";
import { Box, Code, ScrollArea } from "@radix-ui/themes";

const styles = {
  container: css({ borderLeft: "2px solid var(--red-8)" }),
  error: css({ whiteSpace: "pre-wrap", wordBreak: "break-word" }),
};

interface Props {
  error: string;
}

export default ({ error }: Props) => {
  return (
    <ScrollArea>
      <Box pl="2" pr="4" css={styles.container}>
        <Code size="2" variant="ghost" css={styles.error}>
          {error}
        </Code>
      </Box>
    </ScrollArea>
  );
};
