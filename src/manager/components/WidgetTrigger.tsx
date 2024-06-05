import { Flex, Text, Box, Tabs } from "@radix-ui/themes";
import { Result, WidgetConfig } from "../../types/backend";

interface WidgetTriggerProps {
  index: number;
  config: Result<WidgetConfig, string>;
}

export default function WidgetTrigger({ index, config }: WidgetTriggerProps) {
  return (
    <Tabs.Trigger
      value={`tab${index}`}
      css={{
        justifyContent: "start",
        height: "35px",
        // Move the active bar indicator from bottom to left
        "&::before": {
          top: "10%",
          bottom: "10%",
          left: 0,
          right: "unset",
          height: "unset",
          width: "2px",
        },
      }}
    >
      <Flex align="center" gap="3">
        <Box
          width="6px"
          height="6px"
          css={{
            borderRadius: "var(--radius-thumb)",
            backgroundColor: "Ok" in config ? "var(--green-10)" : "var(--red-10)",
            opacity: 1, // TODO: decrease opacity when widget unloaded
          }}
        />
        <Text>{"Ok" in config ? config.Ok.deskulptConf.name : "[ERROR]"}</Text>
      </Flex>
    </Tabs.Trigger>
  );
}
