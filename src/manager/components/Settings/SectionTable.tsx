import { Box, Heading, Table } from "@radix-ui/themes";
import { PropsWithChildren } from "react";

interface SectionTableProps {
  title: string;
}

const SectionTable = ({
  title,
  children,
}: PropsWithChildren<SectionTableProps>) => {
  return (
    <Box
      position="relative"
      mt="2"
      pt="4"
      pb="2"
      px="1"
      css={{
        border: "1px solid var(--gray-a6)",
        borderRadius: "var(--radius-2)",
      }}
    >
      <Box
        position="absolute"
        top="calc(-0.5 * var(--heading-line-height-2))"
        left="2"
        px="2"
        css={{ backgroundColor: "var(--gray-1)" }}
        asChild
      >
        <Heading size="2" color="gray" weight="medium">
          {title}
        </Heading>
      </Box>
      <Table.Root
        size="1"
        css={{
          "--table-cell-padding": "var(--space-1) var(--space-2)",
          "--table-cell-min-height": 0,
          "& tr": { "--table-row-box-shadow": "none" },
          "& th": { width: "120px" },
        }}
      >
        <Table.Body>{children}</Table.Body>
      </Table.Root>
    </Box>
  );
};

export default SectionTable;
