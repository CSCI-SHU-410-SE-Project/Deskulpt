import { Badge, Box, Flex, Heading, ScrollArea, Text } from "@radix-ui/themes";

interface ErrorDisplayProps {
  title: string;
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
          <Flex align="center" gap="1">
            <Badge color="red">Error</Badge>
            <Heading size="2" trim="both">
              {title}
            </Heading>
          </Flex>
          <Text
            size="1"
            css={{
              whiteSpace: "pre",
              fontFamily: "var(--code-font-family)",
            }}
          >
            {error}
          </Text>
        </Flex>
      </Box>
    </ScrollArea>
  );
}
