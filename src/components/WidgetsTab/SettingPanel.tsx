import { InputNumber } from "antd";
import { WidgetSetting } from "../../types";
import * as styles from "./styles";

/**
 * The settings panel in the details of a widget.
 */
export default function SettingPanel(props: {
  setting: WidgetSetting;
  updateSetting: (partialSetting: Partial<WidgetSetting>) => void;
}) {
  const { setting, updateSetting } = props;

  return (
    <table>
      <tbody>
        <tr>
          <td css={styles.tableCaption}>Position</td>
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
