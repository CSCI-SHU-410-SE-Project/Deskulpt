import { Badge, Box, Flex, Heading, ScrollArea, Text } from "@radix-ui/themes";

interface Props {
  id: string;
  error: string;
}

export default ({ id, error }: Props) => {
  return (
    <ScrollArea scrollbars="both" asChild>
      <Box p="2">
        <Flex direction="column" gap="2">
          <Flex align="center" gap="2">
            <Badge color="red">Error</Badge>
            <Heading size="2" trim="both" css={{ whiteSpace: "pre" }}>
              {id}
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
};
