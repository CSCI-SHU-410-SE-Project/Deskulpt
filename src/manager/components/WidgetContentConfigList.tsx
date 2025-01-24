import { Code, DataList, Flex, IconButton, Tooltip } from "@radix-ui/themes";
import { WidgetConfig, WidgetConfigType } from "../../types";
import { WidgetDependencies } from "../components";
import { MdOpenInNew } from "react-icons/md";
import { invokeOpenInWidgetsDir } from "../../core/commands";

interface Props {
  id: string;
  config: Extract<WidgetConfig, { type: WidgetConfigType.VALID }>["content"];
}

/**
 * Component for displaying the widget configurations.
 *
 * This is rendered as a data list, displaying the widget ID, name, entry, and
 * external dependencies.
 */
export default ({ id, config }: Props) => {
  return (
    <DataList.Root size="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item>
        <DataList.Label>ID</DataList.Label>
        <DataList.Value>{id}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Name</DataList.Label>
        <DataList.Value>{config.name}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Entry</DataList.Label>
        <DataList.Value>
          <Flex align="center" gap="2">
            <Code>{config.entry}</Code>
            <Tooltip content="Open" side="right">
              <IconButton
                variant="ghost"
                size="1"
                onClick={() =>
                  invokeOpenInWidgetsDir({
                    components: [id, config.entry],
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
          <WidgetDependencies dependencies={config.dependencies} />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
};
