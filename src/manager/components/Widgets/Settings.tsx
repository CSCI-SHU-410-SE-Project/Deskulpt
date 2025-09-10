import { Flex, Table } from "@radix-ui/themes";
import { LiaTimesSolid } from "react-icons/lia";
import { useSettingsStore } from "../../hooks";
import { memo, useCallback } from "react";
import IntegerInput from "../IntegerInput";
import { css } from "@emotion/react";
import { commands } from "../../../bindings";

const styles = {
  table: css({
    "--table-cell-padding": "var(--space-1) var(--space-2)",
    "--table-cell-min-height": 0,
    "& tr": { "--table-row-box-shadow": "none" },
    "& th": { color: "var(--gray-11)", width: "100px" },
  }),
};

const X = ({ id }: SettingsProps) => {
  const x = useSettingsStore((state) => state.widgets[id].x);
  const onValueChange = useCallback(
    (value: number) =>
      commands.updateSettings({ update: { widget: [id, { x: value }] } }),
    [id],
  );

  return (
    <IntegerInput
      value={x}
      min={0}
      onValueChange={onValueChange}
      width="60px"
    />
  );
};

const Y = ({ id }: SettingsProps) => {
  const y = useSettingsStore((state) => state.widgets[id].y);
  const onValueChange = useCallback(
    (value: number) =>
      commands.updateSettings({ update: { widget: [id, { y: value }] } }),
    [id],
  );

  return (
    <IntegerInput
      value={y}
      min={0}
      onValueChange={onValueChange}
      width="60px"
    />
  );
};

const Opacity = ({ id }: SettingsProps) => {
  const opacity = useSettingsStore((state) => state.widgets[id].opacity);
  const onValueChange = useCallback(
    (value: number) =>
      commands.updateSettings({ update: { widget: [id, { opacity: value }] } }),
    [id],
  );

  return (
    <IntegerInput
      value={opacity}
      min={0}
      max={100}
      onValueChange={onValueChange}
      width="60px"
    />
  );
};

X.displayName = "Settings.X";
Y.displayName = "Settings.Y";
Opacity.displayName = "Settings.Opacity";

interface SettingsProps {
  id: string;
}

const Settings = memo(({ id }: SettingsProps) => {
  return (
    <Table.Root size="1" layout="fixed" css={styles.table}>
      <Table.Body>
        <Table.Row align="center">
          <Table.RowHeaderCell>Position (px)</Table.RowHeaderCell>
          <Table.Cell>
            <Flex gap="1" align="center">
              <X id={id} />
              <LiaTimesSolid size={12} color="var(--gray-11)" />
              <Y id={id} />
            </Flex>
          </Table.Cell>
        </Table.Row>
        <Table.Row align="center">
          <Table.RowHeaderCell>Opacity (%)</Table.RowHeaderCell>
          <Table.Cell>
            <Opacity id={id} />
          </Table.Cell>
        </Table.Row>
      </Table.Body>
    </Table.Root>
  );
});

export default Settings;
