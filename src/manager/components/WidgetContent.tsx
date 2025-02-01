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
import { invokeOpenInWidgetsDir } from "../../commands";
import {
  WidgetConfig,
  WidgetConfigType,
  WidgetSettings,
} from "../../types/backend";
import { emitRenderWidgetToCanvas } from "../../events";
import { Dispatch, SetStateAction } from "react";
import { ManagerWidgetState } from "../../types/frontend";
import WidgetContentHeading from "../components/WidgetContentHeading";
import { toast } from "sonner";
import WidgetContentConfigList from "../components/WidgetContentConfigList";
import WidgetContentSettingsList from "../components/WidgetContentSettingsList";

export interface WidgetContentProps {
  /** The index of the widget in the collection. */
  index: number;
  /** The widget ID. */
  id: string;
  /** The widget configuration. */
  config: WidgetConfig;
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
export default function WidgetContent({
  index,
  id,
  config,
  settings,
  setManagerWidgetStates,
}: WidgetContentProps) {
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
            emitRenderWidgetToCanvas({ id, settings, bundle: true }).then(() =>
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
}
