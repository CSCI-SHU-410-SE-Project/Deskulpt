import { WidgetSettings } from "../../types";
import { emitUpdateSettingsToCanvas } from "../../core/events";
import { DataList, Flex } from "@radix-ui/themes";
import { NumberInput } from "../components";
import { FaTimes } from "react-icons/fa";
import { WidgetsActionType, WidgetsDispatch } from "../hooks";
import { useCallback } from "react";

interface Props {
  /** The widget ID. */
  id: string;
  /** The widget-specific setting. */
  settings: WidgetSettings;
  /** Setter for the manager widget states. */
  widgetsDispatch: WidgetsDispatch;
}

const updateSetting =
  (id: string, widgetsDispatch: WidgetsDispatch) =>
  (partialSettings: Partial<WidgetSettings>) => {
    widgetsDispatch({
      type: WidgetsActionType.SET_SETTINGS,
      payload: { id, settings: partialSettings },
    });
    emitUpdateSettingsToCanvas({ id, settings: partialSettings }).catch(
      console.error,
    );
  };

/**
 * Component for displaying the widget-specific settings.
 *
 * This includes setter for the position and opacity of a widget. The states in the
 * manager will be updated via the setter, and the updated settings will also be sent
 * to the canvas via emitting corresponding events.
 */
export default ({ id, settings, widgetsDispatch }: Props) => {
  const updateX = useCallback(
    (x: number) => {
      updateSetting(id, widgetsDispatch)({ x });
    },
    [id, widgetsDispatch],
  );

  const updateY = useCallback(
    (y: number) => {
      updateSetting(id, widgetsDispatch)({ y });
    },
    [id, widgetsDispatch],
  );

  const updateOpacity = useCallback(
    (opacity: number) => {
      updateSetting(id, widgetsDispatch)({ opacity });
    },
    [id, widgetsDispatch],
  );

  return (
    <DataList.Root size="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item>
        <DataList.Label>Position (px)</DataList.Label>
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
        <DataList.Label>Opacity (%)</DataList.Label>
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
