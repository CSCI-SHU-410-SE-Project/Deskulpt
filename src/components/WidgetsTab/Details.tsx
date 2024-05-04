import { Button } from "antd";
import { Result, WidgetConfig, WidgetSetting } from "../../types";
import { emitRenderWidgetToCanvas } from "../../events";
import ConfigPanel from "./ConfigPanel";
import SettingPanel from "./SettingPanel";
import * as styles from "./styles";
import { css } from "@emotion/react";

/**
 * The details of a widget in the widgets tab.
 */
export default function Details(props: {
  widgetId: string;
  config: Result<WidgetConfig, string>;
  setting: WidgetSetting;
  setSetting: (widgetId: string, setting: WidgetSetting) => void;
}) {
  const { widgetId, config, setting, setSetting } = props;

  return (
    <>
      <div css={[styles.panelWrapper, css({ height: "160px" })]}>
        <div css={styles.panelTitle}>
          <span>
            <strong>Configuration</strong> [{widgetId}]
          </span>
          <Button size="small" onClick={() => console.log("To be done...")}>
            Edit
          </Button>
        </div>
        <ConfigPanel config={config} />
      </div>
      <hr css={{ width: "510px", margin: "10px 0" }} />
      <div css={[styles.panelWrapper, css({ height: "240px" })]}>
        <div css={styles.panelTitle}>
          <strong>Settings</strong>
          <Button
            size="small"
            onClick={() =>
              emitRenderWidgetToCanvas({ widgetId, setting, bundle: true })
            }
          >
            Re-render
          </Button>
        </div>
        <SettingPanel
          setting={setting}
          updateSetting={(partialSetting) => {
            const newSetting = { ...setting, ...partialSetting };
            setSetting(widgetId, newSetting);
            emitRenderWidgetToCanvas({
              widgetId,
              setting: newSetting,
              bundle: false,
            }).catch(console.error);
          }}
        />
      </div>
    </>
  );
}
