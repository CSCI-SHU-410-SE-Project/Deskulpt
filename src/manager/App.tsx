import { useEffect, useState } from "react";
import { ManagerWidgetState } from "../types";
import { Settings } from "../types";
import { useToggleShortcut } from "../hooks/useToggleShortcut";
import { getNewManagerWidgetStates, renderWidgets } from "./utils";
import { useExitAppListener } from "../hooks/useExitAppListener";
import { useUpdateSettingListener } from "../hooks/useUpdateSettingListener";
import WidgetsTab from "../components/WidgetsTab";
import SettingsTab from "../components/SettingsTab";
import LogsTab from "../components/LogsTab";
import AboutTab from "../components/AboutTab";
import { Tabs } from "antd";

/**
 * The main component of the widget manager window.
 */
export default function App(props: { initialSettings: Settings }) {
  const { initialSettings } = props;
  const { toggleShortcut, setToggleShortcut } = useToggleShortcut(
    initialSettings.toggleShortcut,
  );
  const [managerWidgetStates, setManagerWidgetStates] = useState<
    Record<string, ManagerWidgetState>
  >({});

  useExitAppListener(toggleShortcut, managerWidgetStates);
  useUpdateSettingListener(setManagerWidgetStates);

  /**
   * Rescan the widget base directory and render newly added widgets.
   *
   * Newly added widgets are those that exist in the new states but does not exist in
   * the previous states.
   */
  async function rescanAndRender() {
    const newManagerWidgetStates = await getNewManagerWidgetStates(
      managerWidgetStates,
      initialSettings.widgetSettings,
    );
    const addedStates = Object.fromEntries(
      Object.entries(newManagerWidgetStates).filter(
        ([widgetId]) => !(widgetId in managerWidgetStates),
      ),
    );
    setManagerWidgetStates(newManagerWidgetStates); // Direct replacement
    await renderWidgets(addedStates);
  }

  useEffect(() => {
    // The rescan is guaranteed to succeed because it triggers a command in the backend;
    // the rendering, however, mail fail due to the canvas not being ready to receive
    // the rendering events; this should be rare with a 1.5-second timeout
    const timer = setTimeout(() => {
      rescanAndRender().catch(console.error);
    }, 1500);

    return () => {
      clearTimeout(timer);
    };
  }, []);

  const tabItems = [
    {
      key: "1",
      label: "Widgets",
      children: (
        <WidgetsTab
          managerWidgetStates={managerWidgetStates}
          setManagerWidgetStates={setManagerWidgetStates}
          rescanAndRender={rescanAndRender}
        />
      ),
    },
    {
      key: "2",
      label: "Settings",
      children: (
        <SettingsTab
          toggleShortcut={toggleShortcut}
          setToggleShortcut={setToggleShortcut}
        />
      ),
    },
    {
      key: "3",
      label: "Logs",
      children: <LogsTab />,
    },
    {
      key: "4",
      label: "About",
      children: <AboutTab />,
    },
  ];

  return (
    <Tabs
      defaultActiveKey="1"
      type="card"
      items={tabItems}
      css={{
        "& > .ant-tabs-nav > .ant-tabs-nav-wrap > .ant-tabs-nav-list > .ant-tabs-tab": {
          width: "100px",
          justifyContent: "center",
        },
      }}
    />
  );
}
