import { css } from "@emotion/react";
import { Box, Flex, Tabs, Text } from "@radix-ui/themes";
import { memo } from "react";

const styles = {
  trigger: css({
    justifyContent: "start",
    height: "var(--line-height-7)",
    // Move the active bar indicator from bottom to left
    "&::before": {
      top: "10%",
      bottom: "10%",
      left: 0,
      right: "unset",
      height: "unset",
      width: "2px",
    },
  }),
  indicator: css({ borderRadius: "var(--radius-thumb)" }),
};

interface Props {
  value: string;
  isValid: boolean;
  dir: string;
}

const Trigger = memo(({ value, isValid, dir }: Props) => {
  return (
    <Tabs.Trigger value={value} css={styles.trigger}>
      <Flex align="center" gap="3">
        <Box
          width="6px"
          height="6px"
          css={styles.indicator}
          style={{
            backgroundColor: isValid ? "var(--green-10)" : "var(--red-10)",
            opacity: 1, // TODO: decrease opacity when widget unloaded
          }}
        />
        <Text>{dir}</Text>
      </Flex>
    </Tabs.Trigger>
  );
});

export default Trigger;
