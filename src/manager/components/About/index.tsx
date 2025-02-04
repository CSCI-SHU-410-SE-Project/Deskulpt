import { Avatar, DataList, Flex, Heading, Text } from "@radix-ui/themes";
import ExternalCopyLink from "./ExternalCopyLink";
import Logo from "/deskulpt.svg";
import { memo } from "react";

const AboutTab = memo(() => {
  return (
    <Flex height="100%" pb="9" px="3" justify="center" align="center" gap="3">
      <Flex align="center" justify="center" css={{ flex: 1 }}>
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
      <Flex direction="column" gap="3" css={{ flex: 2 }}>
        <Flex direction="column">
          <Heading size="6">Deskulpt</Heading>
          <Text>A cross-platform desktop customization tool</Text>
        </Flex>
        <DataList.Root size="2" css={{ gap: "var(--space-1)" }}>
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
                CSCI-SHU-410-SE-Project/Deskulpt
              </ExternalCopyLink>
            </DataList.Value>
          </DataList.Item>
          <DataList.Item>
            <DataList.Label>Documentation</DataList.Label>
            <DataList.Value>
              <ExternalCopyLink href="https://csci-shu-410-se-project.github.io/Deskulpt/">
                Website
              </ExternalCopyLink>
            </DataList.Value>
          </DataList.Item>
        </DataList.Root>
      </Flex>
    </Flex>
  );
});

export default AboutTab;
