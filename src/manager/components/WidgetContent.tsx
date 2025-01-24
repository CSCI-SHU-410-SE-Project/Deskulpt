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
import { invokeOpenInWidgetsDir } from "../../core/commands";
import { WidgetConfig, WidgetConfigType, WidgetSettings } from "../../types";
import { emitRenderToCanvas } from "../../core/events";
import { toast } from "sonner";
import WidgetContentHeading from "./WidgetContentHeading";
import WidgetContentConfigList from "./WidgetContentConfigList";
import WidgetContentSettingsList from "./WidgetContentSettingsList";
import { WidgetsDispatch } from "../hooks";

interface Props {
  index: number;
  id: string;
  config: WidgetConfig;
  settings: WidgetSettings;
  widgetsDispatch: WidgetsDispatch;
}

/**
 * The widget content component.
 *
 * It is rendered as the content of a tab. It consists of a configuration section
 * {@link WidgetContentConfigList} and a setting section
 * {@link WidgetContentSettingsList}.
 */
export default ({ index, id, config, settings, widgetsDispatch }: Props) => {
  return (
    <Tabs.Content value={`tab${index}`} asChild>
      <Flex height="100%" gap="3" direction="column" css={{ flex: 3 }}>
        <WidgetContentHeading
          heading={
            <Flex align="center" gap="2">
              Configuration{" "}
              {"Err" in config && <Badge color="red">Error</Badge>}
            </Flex>
          }
          actionIcon={<LuFolderOpen />}
          actionText="Edit"
          action={() => invokeOpenInWidgetsDir({ components: [id] })}
        />
        <ScrollArea scrollbars="vertical" asChild>
          <Box px="2" css={{ flex: 3 }}>
            {config.type === WidgetConfigType.VALID ? (
              <WidgetContentConfigList id={id} config={config.content} />
            ) : (
              <Text
                size="1"
                css={{
                  whiteSpace: "pre-wrap",
                  fontFamily: "var(--code-font-family)",
                }}
              >
                {config.content}
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
            emitRenderToCanvas({ id, settings }).then(() =>
              toast.success(`Re-rendered widget "${id}".`),
            )
          }
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
