import {
  Badge,
  Box,
  Flex,
  ScrollArea,
  Separator,
  Tabs,
  Text,
} from "@radix-ui/themes";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { commands, events } from "../../../core";
import { WidgetConfigType } from "../../../types";
import { toast } from "sonner";
import WidgetContentHeading from "./WidgetContentHeading";
import WidgetContentConfigList from "./WidgetContentConfigList";
import WidgetContentSettingsList from "./WidgetContentSettingsList";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";

interface WidgetContentProps {
  index: number;
  id: string;
}

const WidgetContent = memo(({ index, id }: WidgetContentProps) => {
  const config = useWidgetsStore((state) => state.widgets[id].config);
  const settings = useWidgetsStore((state) => state.widgets[id].settings);

  return (
    <Tabs.Content value={`tab${index}`} asChild>
      <Flex
        height="100%"
        direction="column"
        gap="4"
        pl="2"
        css={{ flex: 3, boxShadow: "inset 1px 0 0 0 var(--gray-a5)" }}
      >
        <WidgetContentHeading
          heading={
            <Flex align="center" gap="2">
              Configuration{" "}
              {"Err" in config && <Badge color="red">Error</Badge>}
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
        <Separator size="4" />
        <WidgetContentHeading
          heading="Settings"
          actionIcon={<LuRepeat />}
          actionText="Re-render"
          action={() =>
            events.renderWidgets
              .toCanvas([{ id }])
              .then(() => toast.success("Re-rendered widget."))
          }
        />
        <Box px="2" css={{ flex: 4 }}>
          <WidgetContentSettingsList id={id} settings={settings} />
        </Box>
      </Flex>
    </Tabs.Content>
  );
});

export default WidgetContent;
