import { Badge, Tabs } from "antd";
import { ManagerWidgetState, Result, WidgetConfig, WidgetSetting } from "../../types";
import WidgetInfoPanel from "../WidgetInfoPanel";
import { Dispatch, SetStateAction } from "react";

export default function WidgetsTab(props: {
  managerWidgetStates: Record<string, ManagerWidgetState>;
  setManagerWidgetStates: Dispatch<SetStateAction<Record<string, ManagerWidgetState>>>;
}) {
  const { managerWidgetStates, setManagerWidgetStates } = props;

  function setSetting(widgetId: string, setting: WidgetSetting) {
    setManagerWidgetStates((prev) => ({
      ...prev,
      [widgetId]: { ...prev[widgetId], setting },
    }));
  }

  return (
    <Tabs
      defaultActiveKey="1"
      tabPosition="left"
      tabBarStyle={{ width: "180px" }}
      style={{ height: "420px" }}
      items={Object.entries(managerWidgetStates).map(
        ([widgetId, { config, setting }]) => ({
          key: widgetId,
          label: getLabelFromConfig(config),
          children: (
            <WidgetInfoPanel
              widgetId={widgetId}
              config={config}
              setting={setting}
              setSetting={setSetting}
            />
          ),
        }),
      )}
    />
  );
}

function getLabelFromConfig(config: Result<WidgetConfig, string>) {
  if ("Ok" in config) {
    return <Badge status="success" text={config.Ok.deskulptConf.name} />;
  }
  return <Badge status="error" text="[ERROR]" />;
}
