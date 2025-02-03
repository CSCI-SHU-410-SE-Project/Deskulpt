import { Dispatch, SetStateAction } from "react";
import { WidgetSettings } from "../../types/backend";
import { ManagerWidgetState } from "../../types/frontend";
import { emitUpdateSettingsToCanvas } from "../../events";
import { DataList, Flex } from "@radix-ui/themes";
import NumberInput from "../components/NumberInput";
import { FaTimes } from "react-icons/fa";

interface WidgetContentSettingListProps {
  /** The widget ID. */
  id: string;
  /** The widget-specific setting. */
  settings: WidgetSettings;
  /** Setter for the manager widget states. */
  setManagerWidgetStates: Dispatch<
    SetStateAction<Record<string, ManagerWidgetState>>
  >;
}

/**
 * Component for displaying the widget-specific settings.
 *
 * This includes setter for the position and opacity of a widget. The states in the
 * manager will be updated via the setter, and the updated settings will also be sent
 * to the canvas via emitting corresponding events.
 */
const WidgetContentSettingList = ({
  id,
  settings,
  setManagerWidgetStates,
}: WidgetContentSettingListProps) => {
  function updateSetting(partialSettings: Partial<WidgetSettings>) {
    setManagerWidgetStates((prev) => ({
      ...prev,
      [id]: { ...prev[id], settings: { ...settings, ...partialSettings } },
    }));
    emitUpdateSettingsToCanvas({ id, settings: partialSettings }).catch(
      console.error,
    );
  }

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
              onChange={(value) => updateSetting({ x: value })}
            />
            <FaTimes size={10} />
            <NumberInput
              value={settings.y}
              min={0}
              width="50px"
              onChange={(value) => updateSetting({ y: value })}
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
            onChange={(value) => updateSetting({ opacity: value })}
          />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
};

export default WidgetContentSettingList;
