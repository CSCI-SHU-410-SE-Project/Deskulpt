import { css } from "@emotion/react";
import { Badge, Box, Code, Flex, ScrollArea } from "@radix-ui/themes";

const styles = {
  error: css({ whiteSpace: "pre" }),
};

interface Props {
  id: string;
  error: string;
}

export default ({ id, error }: Props) => {
  return (
    <ScrollArea asChild>
      <Box p="2">
        <Flex direction="column" gap="1">
          <Badge size="2" color="red">
            Error: {id}
          </Badge>
          <Code size="2" variant="ghost" css={styles.error}>
            {error}
          </Code>
        </Flex>
      </Box>
    </ScrollArea>
  );
};
