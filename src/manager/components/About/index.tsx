import { Avatar, Box, Flex, Heading, Table } from "@radix-ui/themes";
import ExternalCopyLink from "../ExternalCopyLink";
import Logo from "/deskulpt.svg";
import { memo } from "react";
import { FaGithub } from "react-icons/fa";

const AboutTab = memo(() => {
  return (
    <Flex height="100%" pb="9" px="3" justify="center" align="center" gap="3">
      <Flex align="center" justify="center" flexGrow="1">
        <Avatar
          src={Logo}
          fallback="D"
          size="8"
          css={{
            ".dark &": {
              filter: "invert(90%) hue-rotate(170deg)",
              opacity: 0.9,
            },
          }}
        />
      </Flex>
      <Box flexGrow="1">
        <Heading size="6" mb="1">
          Deskulpt
        </Heading>
        <Heading size="3" mb="4" weight="regular" color="gray">
          A cross-platform desktop customization tool
        </Heading>
        <Table.Root
          size="1"
          css={{
            "--table-cell-padding": "var(--space-1) 0",
            "--table-cell-min-height": 0,
            "& tr": { "--table-row-box-shadow": "none" },
            "& th": { color: "var(--gray-11)", width: "120px" },
          }}
        >
          <Table.Body>
            <Table.Row align="center">
              <Table.RowHeaderCell>Version</Table.RowHeaderCell>
              <Table.Cell>{__VERSION__}</Table.Cell>
            </Table.Row>
            <Table.Row align="center">
              <Table.RowHeaderCell>Authors</Table.RowHeaderCell>
              <Table.Cell>The Deskulpt Development Team</Table.Cell>
            </Table.Row>
            <Table.Row align="center">
              <Table.RowHeaderCell>Repository</Table.RowHeaderCell>
              <Table.Cell>
                <ExternalCopyLink href="https://github.com/CSCI-SHU-410-SE-Project/Deskulpt">
                  <Flex align="center" gap="1">
                    <FaGithub /> CSCI-SHU-410-SE-Project/Deskulpt
                  </Flex>
                </ExternalCopyLink>
              </Table.Cell>
            </Table.Row>
            <Table.Row>
              <Table.RowHeaderCell>Homepage</Table.RowHeaderCell>
              <Table.Cell>
                <ExternalCopyLink href="https://csci-shu-410-se-project.github.io/Deskulpt">
                  csci-shu-410-se-project.github.io/Deskulpt
                </ExternalCopyLink>
              </Table.Cell>
            </Table.Row>
          </Table.Body>
        </Table.Root>
        {/* <DataList.Root size="2" css={{ gap: "var(--space-1) 0" }}>
          <DataList.Item>
            <DataList.Label>Version</DataList.Label>
            <DataList.Value>{__VERSION__}</DataList.Value>
          </DataList.Item>
          <DataList.Item>
            <DataList.Label>Authors</DataList.Label>
            <DataList.Value>The Deskulpt Development Team</DataList.Value>
          </DataList.Item>
          <DataList.Item>
            <DataList.Label>Repository</DataList.Label>
            <DataList.Value>
              <ExternalCopyLink href="https://github.com/CSCI-SHU-410-SE-Project/Deskulpt">
                <Flex align="center" gap="1">
                  <FaGithub /> CSCI-SHU-410-SE-Project/Deskulpt
                </Flex>
              </ExternalCopyLink>
            </DataList.Value>
          </DataList.Item>
          <DataList.Item>
            <DataList.Label>Homepage</DataList.Label>
            <DataList.Value>
              <ExternalCopyLink href="https://csci-shu-410-se-project.github.io/Deskulpt">
                csci-shu-410-se-project.github.io/Deskulpt
              </ExternalCopyLink>
            </DataList.Value>
          </DataList.Item>
        </DataList.Root> */}
      </Box>
    </Flex>
  );
});

export default AboutTab;
