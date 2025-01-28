import { Box, Flex, ScrollArea, Tabs, Text } from "@radix-ui/themes";
import { WidgetsState } from "../../hooks";
import { WidgetConfigType } from "../../../types";

interface Props {
  widgets: WidgetsState;
}

export default ({ widgets }: Props) => {
  return (
    <Tabs.List
      css={{
        flex: 1,
        height: "100%",
        // Move the shadow of the tab list from bottom to right
        boxShadow: "inset -1px 0 0 0 var(--gray-a5)",
      }}
    >
      <ScrollArea scrollbars="vertical" asChild>
        <Flex direction="column">
          {widgets.map(({ id, config }, index) => (
            <Tabs.Trigger
              key={id}
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
          ))}
        </Flex>
      </ScrollArea>
    </Tabs.List>
  );
};
