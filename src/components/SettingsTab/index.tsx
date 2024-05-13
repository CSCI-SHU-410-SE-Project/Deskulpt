import { Dispatch, SetStateAction, useState } from "react";
import { Button, Drawer } from "antd";
import * as styles from "./styles";
import ToggleShortcutSetter from "./ToggleShortcutSetter";

/**
 * The global settings tab in the manager.
 */
export default function SettingsTab(props: {
  toggleShortcut: string | null;
  setToggleShortcut: Dispatch<SetStateAction<string | null>>;
}) {
  const { toggleShortcut, setToggleShortcut } = props;

  const [toggleShortcutDrawerOpen, setToggleShortcutDrawerOpen] = useState(false);
  const shortcutKey = getShortcutKey(toggleShortcut);

  return (
    <>
      <div css={{ padding: "10px 20px" }}>
        <p css={{ display: "flex", alignItems: "center" }}>
          <span css={styles.caption}>Toggle shortcut:</span>
          <span>
            {toggleShortcut === null ? (
              "Disabled"
            ) : (
              <>
                <kbd css={styles.keyboard}>CmdorCtrl</kbd>
                <kbd css={styles.keyboard}>Shift</kbd>
                <kbd css={styles.keyboard}>{shortcutKey}</kbd>
              </>
            )}
          </span>
          <Button
            css={styles.rowEnd}
            size="small"
            onClick={() => setToggleShortcutDrawerOpen(true)}
          >
            View and Edit
          </Button>
        </p>
      </div>
      <Drawer
        title="Edit toggle shortcut"
        placement="left"
        open={toggleShortcutDrawerOpen}
        closable={true}
        onClose={() => setToggleShortcutDrawerOpen(false)}
        destroyOnClose
      >
        <ToggleShortcutSetter
          initialDisabled={toggleShortcut === null}
          initialShortcutKey={shortcutKey}
          setToggleShortcut={setToggleShortcut}
          closeDrawer={() => setToggleShortcutDrawerOpen(false)}
        />
      </Drawer>
    </>
  );
}

/**
 * Get the key of the shortcut from the full toggle shortcut string.
 */
function getShortcutKey(toggleShortcut: string | null) {
  if (toggleShortcut === null) {
    return undefined;
  }
  const splitterIndex = toggleShortcut.lastIndexOf("+");
  return toggleShortcut.slice(splitterIndex + 1);
}
