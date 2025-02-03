import { WidgetSettings } from "../../../types";
import { emitUpdateSettingsToCanvas } from "../../../core/events";
import { DataList, Flex } from "@radix-ui/themes";
import { NumberInput } from "../../components";
import { FaTimes } from "react-icons/fa";
import { WidgetsActionType, WidgetsDispatch } from "../../hooks";
import { useCallback } from "react";
import { css } from "@emotion/react";

const styles = {
  root: css({ columnGap: 0, rowGap: "var(--space-2)", flex: 1 }),
};

interface Props {
  id: string;
  settings: WidgetSettings;
  widgetsDispatch: WidgetsDispatch;
}

export default ({ id, settings, widgetsDispatch }: Props) => {
  const updateSettings = useCallback(
    (settings: Partial<WidgetSettings>) => {
      widgetsDispatch({
        type: WidgetsActionType.SET_SETTINGS,
        payload: { id, settings },
      });
      emitUpdateSettingsToCanvas({ id, settings }).catch(console.error);
    },
    [id, widgetsDispatch],
  );

  const updateX = useCallback(
    (x: number) => updateSettings({ x }),
    [updateSettings],
  );
  const updateY = useCallback(
    (y: number) => updateSettings({ y }),
    [updateSettings],
  );
  const updateOpacity = useCallback(
    (opacity: number) => updateSettings({ opacity }),
    [updateSettings],
  );

  return (
    <DataList.Root size="2" css={styles.root}>
      <DataList.Item>
        <DataList.Label minWidth="100px">Position (px)</DataList.Label>
        <DataList.Value>
          <Flex gap="1" align="center">
            <NumberInput
              value={settings.x}
              min={0}
              width="50px"
              onChange={updateX}
            />
            <FaTimes size={10} />
            <NumberInput
              value={settings.y}
              min={0}
              width="50px"
              onChange={updateY}
            />
          </Flex>
        </DataList.Value>
      </DataList.Item>
      <DataList.Item>
        <DataList.Label minWidth="100px">Opacity (%)</DataList.Label>
        <DataList.Value>
          <NumberInput
            value={settings.opacity}
            min={1}
            max={100}
            width="50px"
            onChange={updateOpacity}
          />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
};
