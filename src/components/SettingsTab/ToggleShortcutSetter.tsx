import { Button, Select, Switch, message } from "antd";
import { Dispatch, SetStateAction, useState } from "react";
import * as styles from "./styles";

// Alphanumeric options A-Z and 0-9
const options = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").map((key) => ({
  value: key,
  label: key,
}));

/**
 * The drawer content for setting the toggle shortcut.
 */
export default function ToggleShortcutSetter(props: {
  initialDisabled: boolean;
  initialShortcutKey: string | undefined;
  setToggleShortcut: Dispatch<SetStateAction<string | null>>;
  closeDrawer: () => void;
}) {
  const { initialDisabled, initialShortcutKey, setToggleShortcut, closeDrawer } = props;
  const [disabled, setDisabled] = useState(initialDisabled);
  const [shortcutKey, setShortcutKey] = useState(initialShortcutKey);
  const [messageApi, contextHolder] = message.useMessage();

  function submitSetting() {
    if (!disabled && shortcutKey === undefined) {
      void messageApi.open({
        type: "warning",
        content: "You must either disable the shortcut or select a key.",
      });
      return;
    }
    setToggleShortcut(disabled ? null : `CmdorCtrl+Shift+${shortcutKey}`);
    closeDrawer();
  }

  return (
    <>
      {contextHolder}
      <div css={styles.drawerDescription}>
        The toggle shortcut is used for toggling the sinking/floating state of the
        canvas, equivalent to the "Float/Sink" option in the tray menu. Widgets are not
        interactable when the canvas is floated, and the desktop is not interactable
        when the canvas is sunk.
      </div>
      <div css={styles.spreadWrapper}>
        <span>Enable shortcut</span>
        <Switch defaultChecked={disabled} onChange={(v) => setDisabled(v)} />
      </div>
      <div css={styles.spreadWrapper}>
        <span>Shortcut key</span>
        <Select
          css={{ width: "60px" }}
          size="small"
          disabled={disabled}
          defaultValue={shortcutKey}
          options={options}
          onChange={(v) => setShortcutKey(v)}
        />
      </div>
      <div css={{ marginTop: "30px" }}>
        <Button size="small" onClick={submitSetting}>
          Submit
        </Button>
      </div>
    </>
  );
}
