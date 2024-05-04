import { Button, InputNumber, Tooltip } from "antd";
import { Result, WidgetConfig, WidgetSetting } from "../types";
import { emitRenderWidgetToCanvas } from "../events";
import { css } from "@emotion/react";

const panelWidth = "510px";
const tableCaptionStyle = css({
  padding: "0 20px 0 0",
  color: "gray",
});

export default function WidgetInfoPanel(props: {
  widgetId: string;
  config: Result<WidgetConfig, string>;
  setting: WidgetSetting;
  setSetting: (widgetId: string, setting: WidgetSetting) => void;
}) {
  const { widgetId, config, setting, setSetting } = props;

  return (
    <>
      <div
        css={{
          height: "160px",
          width: panelWidth,
          overflowX: "hidden",
        }}
      >
        <div
          css={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            marginBottom: "10px",
          }}
        >
          <span>
            <strong>Configuration</strong> [{widgetId}]
          </span>
          <Button size="small">Edit</Button>
        </div>
        {"Ok" in config ? (
          <ConfigInfoPanel config={config.Ok} />
        ) : (
          <div
            css={{
              height: "120px",
              paddingRight: "5px",
              fontFamily: "monospace",
              whiteSpace: "pre-wrap",
              overflowY: "auto",
              color: "red",
            }}
          >
            {config.Err}
          </div>
        )}
      </div>
      <hr css={{ width: panelWidth, margin: "10px 0" }} />
      <div
        css={{
          height: "240px",
          width: panelWidth,
          overflowX: "hidden",
        }}
      >
        <div
          css={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "center",
            marginBottom: "10px",
          }}
        >
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
        <SettingInfoPanel
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

function ConfigInfoPanel(props: { config: WidgetConfig }) {
  const { config } = props;
  const { deskulptConf, externalDeps } = config;
  const { name, entry } = deskulptConf;

  return (
    <table>
      <tbody>
        <tr>
          <td css={tableCaptionStyle}>Name</td>
          <td>{name}</td>
        </tr>
        <tr>
          <td css={tableCaptionStyle}>Entry</td>
          <td>
            <code>{entry}</code>
          </td>
        </tr>
        <tr>
          <td css={tableCaptionStyle}>Dependencies</td>
          <td>{getExternalDepsRepr(externalDeps)}</td>
        </tr>
      </tbody>
    </table>
  );
}

function getExternalDepsRepr(externalDeps: Record<string, string>) {
  const externalDepsEntries = Object.entries(externalDeps);
  if (externalDepsEntries.length === 0) {
    return "0";
  }

  return (
    <Tooltip
      placement="right"
      title={
        <div
          css={{
            maxHeight: "120px",
            maxWidth: "300px",
            overflow: "auto",
            scrollbarWidth: "none",
          }}
        >
          {externalDepsEntries.map(([dep, version], index) => (
            <div key={index}>
              <code>{dep}</code> {version}
            </div>
          ))}
        </div>
      }
    >
      {externalDepsEntries.length}
    </Tooltip>
  );
}

function SettingInfoPanel(props: {
  setting: WidgetSetting;
  updateSetting: (partialSetting: Partial<WidgetSetting>) => void;
}) {
  const { setting, updateSetting } = props;
  return (
    <table>
      <tbody>
        <tr>
          <td css={tableCaptionStyle}>Position</td>
          <td>
            <InputNumber
              min={0}
              value={setting.x}
              size="small"
              changeOnWheel
              onChange={(v) => {
                if (v !== null) {
                  updateSetting({ x: v });
                }
              }}
            />{" "}
            <InputNumber
              min={0}
              value={setting.y}
              size="small"
              changeOnWheel
              onChange={(v) => {
                if (v !== null) {
                  updateSetting({ y: v });
                }
              }}
            />
          </td>
        </tr>
      </tbody>
    </table>
  );
}
