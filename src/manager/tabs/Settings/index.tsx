import {
  Dialog,
  Flex,
  IconButton,
  ScrollArea,
  Separator,
  Table,
  Text,
} from "@radix-ui/themes";
import { AppSettings } from "../../../types";
import { UpdateShortcutCallback } from "../../hooks";
import { MdInfoOutline } from "react-icons/md";
import { memo, useMemo } from "react";
import ShortcutAction from "./ShortcutAction";
import { css } from "@emotion/react";

const styles = {
  root: css({ height: "420px" }),
  table: css({
    "--table-cell-padding": "var(--space-1) var(--space-2)",
    "--table-cell-min-height": 0,
  }),
  row: css({ "--table-row-box-shadow": "none" }),
};

interface Props {
  appSettings: AppSettings;
  updateShortcut: UpdateShortcutCallback;
}

const Settings = memo(({ appSettings, updateShortcut }: Props) => {
  const sections = useMemo(
    () => [
      {
        sectionTitle: "Keyboard Shortcuts",
        sectionKey: "keyboard-shortcuts",
        items: [
          {
            title: "Toggle Canvas",
            key: "toggle-canvas",
            info: `Toggle canvas click-through, i.e., sink or float the canvas.
            If the canvas is sunk (click-through), you can interact with the
            desktop but not the widgets. If the canvas is floating, you can
            interact with the widgets but not the desktop.`,
            value: appSettings.shortcuts.toggleCanvas ?? "Disabled",
            action: (
              <ShortcutAction
                shortcutKey="toggleCanvas"
                shortcut={appSettings.shortcuts.toggleCanvas}
                updateShortcut={updateShortcut}
              />
            ),
          },
          {
            title: "Show Manager",
            key: "show-manager",
            info: "Show and focus this manager window.",
            value: appSettings.shortcuts.showManager ?? "Disabled",
            action: (
              <ShortcutAction
                shortcutKey="showManager"
                shortcut={appSettings.shortcuts.showManager}
                updateShortcut={updateShortcut}
              />
            ),
          },
        ],
      },
    ],
    [appSettings.shortcuts, updateShortcut],
  );

  return (
    <ScrollArea css={styles.root}>
      <Flex direction="column" gap="4" mt="2" px="4">
        {sections.map(({ sectionTitle, sectionKey, items }) => (
          <Flex key={sectionKey} direction="column" gap="2">
            <Text size="2" color="gray" weight="medium">
              {sectionTitle}
            </Text>
            <Separator size="4" />
            <Table.Root size="1" css={styles.table}>
              <Table.Body>
                {items.map(({ title, key, info, value, action }) => (
                  <Table.Row key={key} align="center" css={styles.row}>
                    <Table.Cell width="0">
                      <Dialog.Root>
                        <Dialog.Trigger>
                          <Flex align="center">
                            <IconButton size="1" variant="ghost">
                              <MdInfoOutline size="16" />
                            </IconButton>
                          </Flex>
                        </Dialog.Trigger>
                        <Dialog.Content
                          size="2"
                          width="500px"
                          aria-describedby={undefined}
                        >
                          <Dialog.Title size="2">
                            {sectionTitle} / {title}
                          </Dialog.Title>
                          <Separator size="4" />
                          <Text size="2" as="div" mt="3">
                            {info}
                          </Text>
                        </Dialog.Content>
                      </Dialog.Root>
                    </Table.Cell>
                    <Table.RowHeaderCell>{title}</Table.RowHeaderCell>
                    <Table.Cell justify="end">{value}</Table.Cell>
                    <Table.Cell width="100px" justify="end">
                      {action}
                    </Table.Cell>
                  </Table.Row>
                ))}
              </Table.Body>
            </Table.Root>
          </Flex>
        ))}
      </Flex>
    </ScrollArea>
  );
});

export default Settings;
