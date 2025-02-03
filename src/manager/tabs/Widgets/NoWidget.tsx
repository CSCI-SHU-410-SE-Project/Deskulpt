import { Flex, Link, Text } from "@radix-ui/themes";
import { memo } from "react";
import { WEBSITE_URL } from "../../consts";
import { css } from "@emotion/react";

const styles = {
  content: css({ whiteSpace: "normal", textAlign: "center" }),
};

const NoWidget = memo(() => {
  return (
    <Flex height="100%" justify="center">
      <Flex
        direction="column"
        justify="center"
        gap="3"
        height="100%"
        width="50%"
        css={styles.content}
      >
        <Text size="3" weight="medium" as="div">
          No widgets found
        </Text>
        <Text size="2" as="div" color="gray">
          Start by adding widgets to the widgets directory and rescanning the
          widgets directory.
        </Text>
        <Text size="2" as="div" color="gray">
          Check out <Link href={WEBSITE_URL}>Deskulpt website</Link> for more
          information.
        </Text>
      </Flex>
    </Flex>
  );
});

export default NoWidget;
