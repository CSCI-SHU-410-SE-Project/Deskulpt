import { Box, Flex, Heading, ScrollArea } from "@radix-ui/themes";

interface ErrorDisplayProps {
  title: string;
  error: string;
}

/**
 * The error display component for user widget errors.
 */
export default function ErrorDisplay({ title, error }: ErrorDisplayProps) {
  return (
    <ScrollArea scrollbars="both">
      <Flex direction="column" gap="1">
        <Heading size="3">{title}</Heading>
        <Box>
          <pre>{error}</pre>
        </Box>
      </Flex>
    </ScrollArea>
  );
}
