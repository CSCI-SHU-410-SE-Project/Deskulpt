import { Avatar, DataList, Flex, Heading, Text } from "@radix-ui/themes";
import { ExternalCopyLink } from "../../components";
import Logo from "/deskulpt.svg";
import { GITHUB_URL, WEBSITE_URL } from "../../consts";
import { memo } from "react";
import { css } from "@emotion/react";

const styles = {
  logo: css({
    ".dark &": {
      filter: "invert(90%) hue-rotate(170deg)",
      opacity: 0.9,
    },
  }),
  dataList: css({ gap: "var(--space-1) 0" }),
};

const About = memo(() => {
  return (
    <Flex height="100%" pb="9" px="3" justify="center" align="center" gap="3">
      <Flex align="center" justify="center" flexGrow="1">
        <Avatar src={Logo} fallback="D" size="8" css={styles.logo} />
      </Flex>
      <Flex direction="column" gap="3" flexGrow="1">
        <Flex direction="column">
          <Heading size="6">Deskulpt</Heading>
          <Text>A cross-platform desktop customization tool</Text>
        </Flex>
        <DataList.Root size="2" css={styles.dataList}>
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
              <ExternalCopyLink href={GITHUB_URL}>
                CSCI-SHU-410-SE-Project/Deskulpt
              </ExternalCopyLink>
            </DataList.Value>
          </DataList.Item>
          <DataList.Item>
            <DataList.Label>Documentation</DataList.Label>
            <DataList.Value>
              <ExternalCopyLink href={WEBSITE_URL}>Website</ExternalCopyLink>
            </DataList.Value>
          </DataList.Item>
        </DataList.Root>
      </Flex>
    </Flex>
  );
});

export default About;
