import { Flex, Table } from "@radix-ui/themes";
import { LiaTimesSolid } from "react-icons/lia";
import { useWidgetsStore } from "../../hooks";
import { commands } from "../../../core";
import { memo, useCallback } from "react";
import IntegerInput from "../IntegerInput";
import { css } from "@emotion/react";

const styles = {
  table: css({
    "--table-cell-padding": "var(--space-1) var(--space-2)",
    "--table-cell-min-height": 0,
    "& tr": { "--table-row-box-shadow": "none" },
    "& th": { color: "var(--gray-11)", width: "100px" },
  }),
};

const X = ({ id }: SettingsProps) => {
  const x = useWidgetsStore((state) => state.widgets[id].settings.x);
  const onValueChange = useCallback(
    (value: number) =>
      commands
        .updateSettings({
          update: { widgets: { [id]: { x: value } } },
        })
        .catch(console.error),
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
  const y = useWidgetsStore((state) => state.widgets[id].settings.y);
  const onValueChange = useCallback(
    (value: number) =>
      commands
        .updateSettings({
          update: { widgets: { [id]: { y: value } } },
        })
        .catch(console.error),
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
  const opacity = useWidgetsStore(
    (state) => state.widgets[id].settings.opacity,
  );
  const onValueChange = useCallback(
    (value: number) =>
      commands
        .updateSettings({
          update: { widgets: { [id]: { opacity: value } } },
        })
        .catch(console.error),
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
