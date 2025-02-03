import { Box, Code, Dialog, ScrollArea, Text } from "@radix-ui/themes";
import { memo } from "react";

interface ErrorDisplayProps {
  id: string;
  error: string;
  message: string;
}

const ErrorDisplay = memo(({ id, error, message }: ErrorDisplayProps) => {
  return (
    <Dialog.Root>
      <Dialog.Trigger>
        <Box width="100%" height="100%" p="2" css={{ cursor: "pointer" }}>
          <Text size="2" color="red">
            An error occurred in widget <Code variant="ghost">{id}</Code>. Click
            anywhere to check the details.
          </Text>
        </Box>
      </Dialog.Trigger>
      <Dialog.Content size="2" maxWidth="60vw">
        <Dialog.Title size="3" color="red" mb="1">
          Error in widget <Code variant="ghost">{id}</Code>
        </Dialog.Title>
        <Dialog.Description size="2" color="red" mb="2">
          {error}
        </Dialog.Description>
        <ScrollArea asChild>
          <Box pb="3" pr="3" maxHeight="50vh">
            <Code size="2" variant="ghost" css={{ whiteSpace: "pre" }}>
              {message}
            </Code>
          </Box>
        </ScrollArea>
      </Dialog.Content>
    </Dialog.Root>
  );
});

export default ErrorDisplay;
