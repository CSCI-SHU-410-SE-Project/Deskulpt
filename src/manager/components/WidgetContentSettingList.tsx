import { Dispatch, SetStateAction } from "react";
import { IdMap, WidgetSetting } from "../../types/backend";
import { ManagerWidgetState } from "../../types/frontend";
import { emitRenderWidgetToCanvas } from "../../events";
import { DataList, Flex } from "@radix-ui/themes";
import NumberInput from "./NumberInput";
import { FaTimes } from "react-icons/fa";

interface WidgetContentSettingListProps {
  widgetId: string;
  setting: WidgetSetting;
  setManagerWidgetStates: Dispatch<SetStateAction<IdMap<ManagerWidgetState>>>;
}

export default function WidgetContentSettingList({
  widgetId,
  setting,
  setManagerWidgetStates,
}: WidgetContentSettingListProps) {
  function updateSetting(partialSetting: Partial<WidgetSetting>) {
    const newSetting = { ...setting, ...partialSetting };
    setManagerWidgetStates((prev) => ({
      ...prev,
      [widgetId]: { ...prev[widgetId], setting: newSetting },
    }));
    emitRenderWidgetToCanvas({
      widgetId,
      setting: newSetting,
      bundle: false,
    }).catch(console.error);
  }

  return (
    <DataList.Root size="2" css={{ gap: "var(--space-2)" }}>
      <DataList.Item>
        <DataList.Label>Position (px)</DataList.Label>
        <DataList.Value>
          <Flex gap="1" align="center">
            <NumberInput
              value={setting.x}
              min={0}
              width="50px"
              onChange={(value) => updateSetting({ x: value })}
            />
            <FaTimes size={10} />
            <NumberInput
              value={setting.y}
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
            value={setting.opacity}
            min={1}
            max={100}
            width="50px"
            onChange={(value) => updateSetting({ opacity: value })}
          />
        </DataList.Value>
      </DataList.Item>
    </DataList.Root>
  );
}
