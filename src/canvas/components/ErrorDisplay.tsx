import { Box, Code, Flex, Heading, ScrollArea } from "@radix-ui/themes";
import { memo } from "react";

interface ErrorDisplayProps {
  id: string;
  error: string;
}

const ErrorDisplay = memo(({ id, error }: ErrorDisplayProps) => {
  return (
    <ScrollArea scrollbars="both" asChild>
      <Box p="2">
        <Flex direction="column" gap="2">
          <Heading size="2" color="red" css={{ whiteSpace: "pre" }}>
            {id}-{id}
          </Heading>
          <Code size="2" variant="ghost" css={{ whiteSpace: "pre" }}>
            {error}
          </Code>
        </Flex>
      </Box>
    </ScrollArea>
  );
});

export default ErrorDisplay;
