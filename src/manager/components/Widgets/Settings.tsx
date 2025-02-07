import { Flex, Table } from "@radix-ui/themes";
import { FaTimes } from "react-icons/fa";
import { updateWidgetSettings, useWidgetsStore } from "../../hooks";
import { memo, useCallback } from "react";
import NumberInput from "./NumberInput";

const X = ({ id }: SettingsProps) => {
  const x = useWidgetsStore((state) => state.widgets[id].settings.x);
  const onChange = useCallback(
    (value: number) => updateWidgetSettings(id, { x: value }, true),
    [id],
  );

  return <NumberInput value={x} min={0} width="50px" onChange={onChange} />;
};

const Y = ({ id }: SettingsProps) => {
  const y = useWidgetsStore((state) => state.widgets[id].settings.y);
  const onChange = useCallback(
    (value: number) => updateWidgetSettings(id, { y: value }, true),
    [id],
  );

  return <NumberInput value={y} min={0} width="50px" onChange={onChange} />;
};

const Opacity = ({ id }: SettingsProps) => {
  const opacity = useWidgetsStore(
    (state) => state.widgets[id].settings.opacity,
  );
  const onChange = useCallback(
    (value: number) => updateWidgetSettings(id, { opacity: value }, true),
    [id],
  );

  return (
    <NumberInput value={opacity} min={0} width="50px" onChange={onChange} />
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
    <Table.Root
      size="1"
      variant="ghost"
      layout="fixed"
      css={{
        "--table-cell-padding": "var(--space-1) var(--space-2)",
        "--table-cell-min-height": 0,
        "& tr": { "--table-row-box-shadow": "none" },
        // Did not use <th> because we may have another column in the future,
        // so header is not semantically correct
        "& .label": { color: "var(--gray-11)", width: "100px" },
      }}
    >
      <Table.Body>
        <Table.Row align="center">
          <Table.Cell className="label">Position (px)</Table.Cell>
          <Table.Cell>
            <Flex gap="1" align="center">
              <X id={id} />
              <FaTimes size={12} color="var(--gray-11)" />
              <Y id={id} />
            </Flex>
          </Table.Cell>
        </Table.Row>
        <Table.Row align="center">
          <Table.Cell className="label">Opacity (%)</Table.Cell>
          <Table.Cell>
            <Opacity id={id} />
          </Table.Cell>
        </Table.Row>
      </Table.Body>
    </Table.Root>
  );
});

export default Settings;
