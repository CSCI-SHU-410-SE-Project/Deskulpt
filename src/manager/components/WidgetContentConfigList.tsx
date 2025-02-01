import { Code, DataList, Flex, IconButton, Tooltip } from "@radix-ui/themes";
import { WidgetConfig, WidgetConfigType } from "../../types/backend";
import WidgetDependencies from "../components/WidgetDependencies";
import { MdOpenInNew } from "react-icons/md";
import { invokeOpenInWidgetsDir } from "../../commands";

export interface WidgetContentConfigListProps {
  /** The widget ID. */
  id: string;
  /** The widget configuration. */
  config: Extract<WidgetConfig, { type: WidgetConfigType.VALID }>;
}

/**
 * Component for displaying the widget configurations.
 *
 * This is rendered as a data list, displaying the widget ID, name, entry, and
 * external dependencies.
 */
export default function WidgetContentConfigList({
  id,
  config,
}: WidgetContentConfigListProps) {
  return (
    <DataList.Root size="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item>
        <DataList.Label>ID</DataList.Label>
        <DataList.Value>{id}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Name</DataList.Label>
        <DataList.Value>{config.content.name}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Entry</DataList.Label>
        <DataList.Value>
          <Flex align="center" gap="2">
            <Code>{config.content.entry}</Code>
            <Tooltip content="Open" side="right">
              <IconButton
                variant="ghost"
                size="1"
                onClick={() =>
                  invokeOpenInWidgetsDir({
                    components: [config.content.dir, config.content.entry],
                  })
                }
              >
                <MdOpenInNew />
              </IconButton>
            </Tooltip>
          </Flex>
        </DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Dependencies</DataList.Label>
        <DataList.Value>
          <WidgetDependencies dependencies={config.content.dependencies} />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
}
