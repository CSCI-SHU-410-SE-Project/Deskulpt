import { Flex, IconButton, Popover, Table, Text } from "@radix-ui/themes";
import { PropsWithChildren } from "react";
import { MdInfoOutline } from "react-icons/md";

const InfoCell = ({ children }: PropsWithChildren) => {
  return (
    <Table.Cell width="0">
      <Popover.Root>
        <Popover.Trigger>
          <Flex align="center">
            <IconButton size="1" variant="ghost">
              <MdInfoOutline size="16" />
            </IconButton>
          </Flex>
        </Popover.Trigger>
        <Popover.Content size="1">
          <Text size="2">{children}</Text>
        </Popover.Content>
      </Popover.Root>
    </Table.Cell>
  );
};

export default InfoCell;
