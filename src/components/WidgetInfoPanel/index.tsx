import { Button, InputNumber, Tooltip } from "antd";
import { Result, WidgetConfig, WidgetSetting } from "../../types";
import * as Styled from "./styled";
import { emitRenderWidgetToCanvas } from "../../events";

export default function WidgetInfoPanel(props: {
  widgetId: string;
  config: Result<WidgetConfig, string>;
  setting: WidgetSetting;
  setSetting: (widgetId: string, setting: WidgetSetting) => void;
}) {
  const { widgetId, config, setting, setSetting } = props;

  return (
    <>
      <Styled.UpperPanel>
        <Styled.PanelSectionHeading>
          <span>
            <strong>Configuration</strong> [{widgetId}]
          </span>
          <Button size="small">Edit</Button>
        </Styled.PanelSectionHeading>
        {"Ok" in config ? (
          <ConfigInfoPanel config={config.Ok} />
        ) : (
          <Styled.ConfigErrorInfo>{config.Err}</Styled.ConfigErrorInfo>
        )}
      </Styled.UpperPanel>
      <Styled.PanelDivider />
      <Styled.LowerPanel>
        <Styled.PanelSectionHeading>
          <strong>Settings</strong>
          <Button
            size="small"
            onClick={() =>
              emitRenderWidgetToCanvas({ widgetId, setting, bundle: true })
            }
          >
            Re-render
          </Button>
        </Styled.PanelSectionHeading>
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
      </Styled.LowerPanel>
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
          <Styled.TableInfoCell>Name</Styled.TableInfoCell>
          <td>{name}</td>
        </tr>
        <tr>
          <Styled.TableInfoCell>Entry</Styled.TableInfoCell>
          <td>
            <code>{entry}</code>
          </td>
        </tr>
        <tr>
          <Styled.TableInfoCell>Dependencies</Styled.TableInfoCell>
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
        <Styled.TooltipWrapper>
          {externalDepsEntries.map(([dep, version], index) => (
            <div key={index}>
              <code>{dep}</code> {version}
            </div>
          ))}
        </Styled.TooltipWrapper>
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
          <Styled.TableInfoCell>Position</Styled.TableInfoCell>
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
