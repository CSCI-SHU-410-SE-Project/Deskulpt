import { Badge, Box, Flex, Heading, ScrollArea, Text } from "@radix-ui/themes";

export interface ErrorDisplayProps {
  /**
   * Title of the error display.
   *
   * This should be a short description of the error. It will be displayed as a heading.
   */
  title: string;
  /**
   * The full error stack.
   *
   * This will be displayed as a pre-wrap code block.
   */
  error: string;
}

/**
 * The error display component for user widget errors.
 */
export default function ErrorDisplay({ title, error }: ErrorDisplayProps) {
  return (
    <ScrollArea scrollbars="both" asChild>
      <Box p="2">
        <Flex direction="column" gap="2">
          <Flex align="center" gap="2">
            <Badge color="red">Error</Badge>
            <Heading size="2" trim="both" css={{ whiteSpace: "pre" }}>
              {title}
            </Heading>
          </Flex>
          <Text
            size="1"
            css={{ whiteSpace: "pre", fontFamily: "var(--code-font-family)" }}
          >
            {error}
          </Text>
        </Flex>
      </Box>
    </ScrollArea>
  );
}
