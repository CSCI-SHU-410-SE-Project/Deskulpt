import { Badge, Box, Flex, ScrollArea, Text } from "@radix-ui/themes";
import { LuFolderOpen } from "react-icons/lu";
import { commands } from "../../../core";
import { WidgetConfigType } from "../../../types";
import WidgetContentHeading from "./WidgetContentHeading";
import WidgetContentConfigList from "./WidgetContentConfigList";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";

interface ConfigProps {
  id: string;
}

const Config = memo(({ id }: ConfigProps) => {
  const config = useWidgetsStore((state) => state.widgets[id].config);

  return (
    <>
      <WidgetContentHeading
        heading={
          <Flex align="center" gap="2">
            Configuration {"Err" in config && <Badge color="red">Error</Badge>}
          </Flex>
        }
        actionIcon={<LuFolderOpen />}
        actionText="Edit"
        action={() =>
          commands.openInWidgetsDir({ components: [config.content.dir] })
        }
      />
      <ScrollArea scrollbars="vertical" asChild>
        <Box px="2" css={{ flex: 3 }}>
          {config.type === WidgetConfigType.VALID ? (
            <WidgetContentConfigList id={id} config={config} />
          ) : (
            <Text
              size="1"
              css={{
                whiteSpace: "pre-wrap",
                fontFamily: "var(--code-font-family)",
              }}
            >
              {config.content.error}
            </Text>
          )}
        </Box>
      </ScrollArea>
    </>
  );
});

export default Config;
