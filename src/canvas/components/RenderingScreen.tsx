import { Avatar, Flex, Text } from "@radix-ui/themes";
import Logo from "/deskulpt.svg";
import { css, keyframes } from "@emotion/react";

const rotate = keyframes({
  "0%": { transform: "rotate(0deg)" },
  "100%": { transform: "rotate(360deg)" },
});

const styles = {
  root: css({ zIndex: 10000 }),
  container: css({
    color: "var(--gray-12)",
    backgroundColor: "var(--gray-surface)",
    borderRadius: "var(--radius-2)",
    boxShadow: "0 0 2px var(--gray-8)",
  }),
  logo: css({
    animation: `${rotate} 1.2s cubic-bezier(0.25, 0.1, 0.25, 1) infinite`,
    ".dark &": {
      filter: "invert(90%) hue-rotate(170deg)",
      opacity: 0.9,
    },
  }),
};

export default () => {
  return (
    <Flex
      align="center"
      justify="center"
      width="100%"
      height="100%"
      css={styles.root}
    >
      <Flex
        direction="column"
        align="center"
        px="6"
        py="4"
        gap="2"
        css={styles.container}
      >
        <Avatar src={Logo} fallback="D" size="4" css={styles.logo} />
        <Text size="2">Preparing your widgets...</Text>
      </Flex>
    </Flex>
  );
};
