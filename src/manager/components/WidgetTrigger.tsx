import { Badge, Box, Flex, Tabs, Text } from "@radix-ui/themes";
import { Result, WidgetConfig } from "../../types/backend";

export interface WidgetTriggerProps {
  /** The index of the widget in the collection. */
  index: number;
  /** The widget configuration or error. */
  config: Result<WidgetConfig, string>;
}

/**
 * The widget tab trigger component.
 *
 * It will be rendered as a trigger in the tab list. If the configuration is valid, it
 * will display the widget name with a green indicator. Otherwise, it will display an
 * error badge with a red indicator.
 */
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
        <Text>
          {"Ok" in config ? (
            config.Ok.deskulptConf.name
          ) : (
            <Badge color="red">Error</Badge>
          )}
        </Text>
      </Flex>
    </Tabs.Trigger>
  );
}
