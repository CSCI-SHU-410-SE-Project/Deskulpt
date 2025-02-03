import { DataList } from "@radix-ui/themes";
import Dependencies from "./Dependencies";
import { memo } from "react";
import { css } from "@emotion/react";

const styles = {
  root: css({ columnGap: 0, rowGap: "var(--space-2)" }),
  value: css({
    whiteSpace: "nowrap",
    overflowX: "auto",
    scrollbarWidth: "none",
  }),
};

interface Props {
  name: string;
  entry: string;
  dependencies: Record<string, string>;
}

const Config = memo(({ name, entry, dependencies }: Props) => {
  return (
    <DataList.Root size="2" css={styles.root}>
      <DataList.Item>
        <DataList.Label>Name</DataList.Label>
        <DataList.Value css={styles.value}>{name}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Entry</DataList.Label>
        <DataList.Value css={styles.value}>{entry}</DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label>Dependencies</DataList.Label>
        <DataList.Value>
          <Dependencies deps={dependencies} />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
});

export default Config;
