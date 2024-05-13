import { Badge, FloatButton, Tabs } from "antd";
import { ManagerWidgetState } from "../../types/frontend";
import { Result, WidgetConfig, WidgetSetting } from "../../types/backend";
import WidgetInfoPanel from "./Details";
import { Dispatch, SetStateAction } from "react";
import { FileScan, FolderOpen, Repeat } from "lucide-react";
import { invokeOpenWidgetDirectory } from "../../commands";
import { renderWidgets } from "../../manager/utils";

/**
 * The widgets tab in the manager.
 */
export default function WidgetsTab(props: {
  managerWidgetStates: Record<string, ManagerWidgetState>;
  setManagerWidgetStates: Dispatch<SetStateAction<Record<string, ManagerWidgetState>>>;
  rescanAndRender: () => Promise<void>;
}) {
  const { managerWidgetStates, setManagerWidgetStates, rescanAndRender } = props;

  function setSetting(widgetId: string, setting: WidgetSetting) {
    setManagerWidgetStates((prev) => ({
      ...prev,
      [widgetId]: { ...prev[widgetId], setting },
    }));
  }

  const tabItems = Object.entries(managerWidgetStates).map(
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
  );

  return (
    <>
      <Tabs
        defaultActiveKey="1"
        tabPosition="left"
        items={tabItems}
        css={{
          height: "420px",
          ".ant-tabs-nav": {
            width: "180px",
          },
        }}
      />
      <FloatButton
        css={{ bottom: "130px" }}
        icon={<Repeat size={15} />}
        tooltip="Re-render all widgets"
        onClick={() => renderWidgets(managerWidgetStates)}
      />
      <FloatButton
        css={{ bottom: "80px" }}
        icon={<FileScan size={15} />}
        tooltip="Rescan widgets"
        onClick={rescanAndRender}
      />
      <FloatButton
        css={{ bottom: "30px" }}
        icon={<FolderOpen size={15} />}
        tooltip="Open base directory"
        onClick={() => invokeOpenWidgetDirectory(null)}
      />
    </>
  );
}

/**
 * The label displayed in the widget list.
 */
function getLabelFromConfig(config: Result<WidgetConfig, string>) {
  if ("Ok" in config) {
    return <Badge status="success" text={config.Ok.deskulptConf.name} />;
  }
  return <Badge status="error" text="[ERROR]" />;
}
