import { Badge, Box, Flex, Heading, ScrollArea, Text } from "@radix-ui/themes";

export interface ErrorDisplayProps {
  /** Title of the error display. */
  title: string;
  /** The full error message. */
  error: string;
}

/**
 * The error display component for user widget errors.
 *
 * It will display an error badge and the error title as a heading, followed by the full
 * error message displayed as pre-wrap monospace text. The component is wrapped in a
 * scroll area is scrollable in both directions.
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
