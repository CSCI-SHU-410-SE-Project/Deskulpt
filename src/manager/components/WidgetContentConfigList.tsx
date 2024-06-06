import { Code, DataList, Flex, IconButton, Tooltip } from "@radix-ui/themes";
import { WidgetConfig } from "../../types/backend";
import WidgetDependencies from "./WidgetDependencies";
import { MdOpenInNew } from "react-icons/md";
import { invokeOpenWidgetResource } from "../../commands";

interface WidgetContentConfigListProps {
  widgetId: string;
  config: WidgetConfig;
}

export default function WidgetContentConfigList({
  widgetId,
  config,
}: WidgetContentConfigListProps) {
  return (
    <DataList.Root size="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item>
        <DataList.Label>ID</DataList.Label>
        <DataList.Value>{widgetId}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Name</DataList.Label>
        <DataList.Value>{config.deskulptConf.name}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Entry</DataList.Label>
        <DataList.Value>
          <Flex align="center" gap="2">
            <Code>{config.deskulptConf.entry}</Code>
            <Tooltip content="Open" side="right">
              <IconButton
                variant="ghost"
                size="1"
                onClick={() =>
                  invokeOpenWidgetResource(widgetId, config.deskulptConf.entry)
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
          <WidgetDependencies dependencies={config.externalDeps} />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
}
