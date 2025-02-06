import { Box, Code, ScrollArea, Table } from "@radix-ui/themes";
import { WidgetConfigType } from "../../../types";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";
import Dependencies from "./Dependencies";

interface ConfigProps {
  id: string;
}

const Config = memo(({ id }: ConfigProps) => {
  const config = useWidgetsStore((state) => state.widgets[id].config);

  return (
    <ScrollArea asChild>
      <Box height="160px" pr="3" pb="3">
        {config.type === WidgetConfigType.VALID ? (
          <Table.Root
            size="1"
            variant="ghost"
            layout="fixed"
            css={{
              "--table-cell-padding": "var(--space-1) var(--space-2)",
              "--table-cell-min-height": 0,
              "& tr": { "--table-row-box-shadow": "none" },
              "& th": { color: "var(--gray-11)", width: "120px" },
            }}
          >
            <Table.Body>
              <Table.Row>
                <Table.RowHeaderCell>Name</Table.RowHeaderCell>
                <Table.Cell>{config.content.name}</Table.Cell>
              </Table.Row>
              <Table.Row>
                <Table.RowHeaderCell>Entry</Table.RowHeaderCell>
                <Table.Cell>{config.content.entry}</Table.Cell>
              </Table.Row>
              <Table.Row>
                <Table.RowHeaderCell>Dependencies</Table.RowHeaderCell>
                <Table.Cell>
                  <Dependencies dependencies={config.content.dependencies} />
                </Table.Cell>
              </Table.Row>
            </Table.Body>
          </Table.Root>
        ) : (
          <Box pl="2" m="0" asChild>
            <pre>
              <Code size="2" variant="ghost">
                {config.content.error}
              </Code>
            </pre>
          </Box>
        )}
      </Box>
    </ScrollArea>
  );
});

export default Config;
