import { Box, Flex, Tabs, Text } from "@radix-ui/themes";
import { WidgetConfigType } from "../../../types";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";

interface TriggerProps {
  id: string;
  value: string;
}

const Trigger = memo(({ id, value }: TriggerProps) => {
  const config = useWidgetsStore((state) => state.widgets[id].config);

  return (
    <Tabs.Trigger
      value={value}
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
            backgroundColor:
              config.type === WidgetConfigType.VALID
                ? "var(--green-10)"
                : "var(--red-10)",
            opacity: 1, // TODO: decrease opacity when widget unloaded
          }}
        />
        <Text>{config.content.dir}</Text>
      </Flex>
    </Tabs.Trigger>
  );
});

export default Trigger;
