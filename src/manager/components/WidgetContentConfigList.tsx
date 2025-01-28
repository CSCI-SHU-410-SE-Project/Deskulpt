import {
  Code,
  DataList,
  Flex,
  IconButton,
  Text,
  Tooltip,
} from "@radix-ui/themes";
import { WidgetConfig, WidgetConfigType } from "../../types";
import { WidgetDependencies } from "../components";
import { MdOpenInNew } from "react-icons/md";
import { invokeOpenInWidgetsDir } from "../../core/commands";
import { useCallback } from "react";

interface Props {
  id: string;
  config: WidgetConfig;
}

/**
 * Component for displaying the widget configurations.
 *
 * This is rendered as a data list, displaying the widget ID, name, entry, and
 * external dependencies.
 */
export default ({ id, config }: Props) => {
  const openDirAction = useCallback(() => {
    invokeOpenInWidgetsDir({ components: [id] });
  }, [id]);

  if (config.type === WidgetConfigType.INVALID) {
    return (
      <Text
        size="1"
        css={{
          whiteSpace: "pre-wrap",
          fontFamily: "var(--code-font-family)",
        }}
      >
        {config.content.error}
      </Text>
    );
  }

  const { name, entry, dependencies } = config.content;
  return (
    <DataList.Root size="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item>
        <DataList.Label>ID</DataList.Label>
        <DataList.Value>{id}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Name</DataList.Label>
        <DataList.Value>{name}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Entry</DataList.Label>
        <DataList.Value>
          <Flex align="center" gap="2">
            <Code>{entry}</Code>
            <Tooltip content="Open" side="right">
              <IconButton variant="ghost" size="1" onClick={openDirAction}>
                <MdOpenInNew />
              </IconButton>
            </Tooltip>
          </Flex>
        </DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Dependencies</DataList.Label>
        <DataList.Value>
          <WidgetDependencies dependencies={dependencies} />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
};
