import {
  Avatar,
  Badge,
  DataList,
  Flex,
  Heading,
  Link,
  Separator,
  Text,
} from "@radix-ui/themes";
import ExternalCopyLink from "../components/ExternalCopyLink";
import Logo from "/deskulpt.svg";

/**
 * The about tab in the manager.
 */
export default function AboutTab() {
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
      <Flex direction="column" gap="3" css={{ flex: 3 }}>
        <Flex direction="column">
          <Heading size="6">Deskulpt</Heading>
          <Text>A cross-platform desktop customization tool</Text>
        </Flex>
        <DataList.Root size="2" css={{ gap: "var(--space-1)" }}>
          <DataList.Item>
            <DataList.Label>Version</DataList.Label>
            <DataList.Value>
              <Flex gap="3" align="center">
                <Text>0.0.1</Text>
                <Badge color="red">alpha</Badge>
              </Flex>
            </DataList.Value>
          </DataList.Item>
          <DataList.Item>
            <DataList.Label>Authors</DataList.Label>
            <DataList.Value>
              <Flex gap="2" align="center">
                <Link
                  href="https://github.com/Xinyu-Li-123"
                  target="_blank"
                  rel="noreferrer"
                >
                  Xinyu Li
                </Link>
                <Separator orientation="vertical" />
                <Link
                  href="https://github.com/Charlie-XIAO"
                  target="_blank"
                  rel="noreferrer"
                >
                  Yao Xiao
                </Link>
                <Separator orientation="vertical" />
                <Link
                  href="https://github.com/ROMEEZHOU"
                  target="_blank"
                  rel="noreferrer"
                >
                  Yuchen Zhou
                </Link>
                <Separator orientation="vertical" />
                <Link href="https://github.com/rz2606" target="_blank" rel="noreferrer">
                  Runkai Zhu
                </Link>
              </Flex>
            </DataList.Value>
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
}
