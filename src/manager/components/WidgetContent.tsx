import { Box, Flex, ScrollArea, Separator, Tabs } from "@radix-ui/themes";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { invokeOpenInWidgetsDir } from "../../core/commands";
import { WidgetConfig, WidgetSettings } from "../../types";
import { emitRenderToCanvas } from "../../core/events";
import { toast } from "sonner";
import WidgetContentHeading from "./WidgetContentHeading";
import WidgetContentConfigList from "./WidgetContentConfigList";
import WidgetContentSettingsList from "./WidgetContentSettingsList";
import { WidgetsDispatch } from "../hooks";
import { useCallback } from "react";

interface Props {
  value: string;
  id: string;
  config: WidgetConfig;
  settings: WidgetSettings;
  widgetsDispatch: WidgetsDispatch;
}

export default ({ value, id, config, settings, widgetsDispatch }: Props) => {
  const openWidgetsDirAction = useCallback(() => {
    invokeOpenInWidgetsDir({ components: [id] });
  }, [id]);

  const rerenderAction = useCallback(() => {
    emitRenderToCanvas([{ id, settings }]).then(() =>
      toast.success(`Re-rendered widget "${id}".`),
    );
  }, [id, settings]);

  return (
    <Tabs.Content value={value} asChild>
      <Flex height="100%" gap="3" direction="column" css={{ flex: 3 }}>
        <WidgetContentHeading
          heading="Configuration"
          actionIcon={<LuFolderOpen />}
          actionText="Edit"
          action={openWidgetsDirAction}
        />
        <ScrollArea scrollbars="vertical" asChild>
          <Box px="2" css={{ flex: 3 }}>
            <WidgetContentConfigList id={id} config={config} />
          </Box>
        </ScrollArea>
        <Separator size="4" />
        <WidgetContentHeading
          heading="Settings"
          actionIcon={<LuRepeat />}
          actionText="Re-render"
          action={rerenderAction}
        />
        <Box px="2" css={{ flex: 4 }}>
          <WidgetContentSettingsList
            id={id}
            settings={settings}
            widgetsDispatch={widgetsDispatch}
          />
        </Box>
      </Flex>
    </Tabs.Content>
  );
};
