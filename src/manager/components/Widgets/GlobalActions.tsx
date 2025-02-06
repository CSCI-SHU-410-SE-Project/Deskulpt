import { Flex, IconButton } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { LuFileScan, LuFolderOpen, LuRepeat } from "react-icons/lu";
import { commands, events } from "../../../core";
import { rescan, useWidgetsStore } from "../../hooks";
import { toast } from "sonner";

interface GlobalActionsProps {
  length: number;
}

const GlobalActions = memo(({ length }: GlobalActionsProps) => {
  const refreshAction = useCallback(() => {
    const payload = Object.entries(useWidgetsStore.getState().widgets).map(
      ([id, { settings }]) => ({ id, settings }),
    );
    events.renderWidgets
      .toCanvas(payload)
      .then(() => {
        toast.success(`Refreshed ${payload.length} widgets.`);
      })
      .catch(console.error);
  }, []);

  const rescanAction = useCallback(() => {
    rescan()
      .then((count) => {
        toast.success(`${count} added.`);
      })
      .catch(console.error);
  }, []);

  const openAction = useCallback(() => {
    commands.openWidget().catch(console.error);
  }, []);

  return (
    <Flex gap="6" align="center" justify="center" pb="2" pr="4">
      <IconButton
        title="Refresh current widgets"
        size="1"
        variant="ghost"
        onClick={refreshAction}
        disabled={length === 0}
      >
        <LuRepeat size="16" />
      </IconButton>
      <IconButton
        title="Rescan widgets directory"
        size="1"
        variant="ghost"
        onClick={rescanAction}
      >
        <LuFileScan size="16" />
      </IconButton>
      <IconButton
        title="Open widgets directory"
        size="1"
        variant="ghost"
        onClick={openAction}
      >
        <LuFolderOpen size="16" />
      </IconButton>
    </Flex>
  );
});

export default GlobalActions;
