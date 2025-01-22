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
import { Result, WidgetConfig, WidgetSettings } from "../../types/backend";
import { emitRenderToCanvas } from "../../core/events";
import { Dispatch, SetStateAction } from "react";
import { ManagerWidgetState } from "../../types/frontend";
import WidgetContentHeading from "../components/WidgetContentHeading";
import { toast } from "sonner";
import WidgetContentConfigList from "../components/WidgetContentConfigList";
import WidgetContentSettingsList from "../components/WidgetContentSettingsList";

interface Props {
  /** The index of the widget in the collection. */
  index: number;
  /** The widget ID. */
  id: string;
  /** The widget configuration or error. */
  config: Result<WidgetConfig, string>;
  /** The widget-specific settings. */
  settings: WidgetSettings;
  /** Setter for the manager widget states. */
  setManagerWidgetStates: Dispatch<
    SetStateAction<Record<string, ManagerWidgetState>>
  >;
}

/**
 * The widget content component.
 *
 * It is rendered as the content of a tab. It consists of a configuration section
 * {@link WidgetContentConfigList} and a setting section
 * {@link WidgetContentSettingsList}.
 */
export default ({
  index,
  id,
  config,
  settings,
  setManagerWidgetStates,
}: Props) => {
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
            {"Ok" in config ? (
              <WidgetContentConfigList id={id} config={config.Ok} />
            ) : (
              <Text
                size="1"
                css={{
                  whiteSpace: "pre-wrap",
                  fontFamily: "var(--code-font-family)",
                }}
              >
                {config.Err}
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
            setManagerWidgetStates={setManagerWidgetStates}
          />
        </Box>
      </Flex>
    </Tabs.Content>
  );
};
