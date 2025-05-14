import { Box, Flex, ScrollArea, Table } from "@radix-ui/themes";
import { memo } from "react";
import Shortcut from "./Shortcut";
import InfoCell from "./InfoCell";
import SectionTable from "./SectionTable";
import { ShortcutKey } from "../../../types";

const Settings = memo(() => {
  return (
    <ScrollArea asChild>
      <Box height="420px" mt="1" pl="1" pr="3">
        <Flex direction="column" gap="4">
          <SectionTable title="Keyboard Shortcuts">
            <Table.Row align="center">
              <InfoCell>
                Toggle canvas click-through, i.e., sink or float the canvas. If
                the canvas is sunk (click-through), you can interact with the
                desktop but not the widgets. If the canvas is floating (not
                click-through), you can interact with the widgets but not the
                desktop.
              </InfoCell>
              <Table.RowHeaderCell>Toggle Canvas</Table.RowHeaderCell>
              <Table.Cell>
                <Shortcut shortcutKey={ShortcutKey.TOGGLE_CANVAS} />
              </Table.Cell>
            </Table.Row>
            <Table.Row align="center">
              <InfoCell>Open this manager window.</InfoCell>
              <Table.RowHeaderCell>Open Manager</Table.RowHeaderCell>
              <Table.Cell>
                <Shortcut shortcutKey={ShortcutKey.OPEN_MANAGER} />
              </Table.Cell>
            </Table.Row>
          </SectionTable>
        </Flex>
      </Box>
    </ScrollArea>
  );
});

export default Settings;
