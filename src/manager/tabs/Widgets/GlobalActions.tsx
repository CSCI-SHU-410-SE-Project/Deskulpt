import { Flex, IconButton } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { RescanCallback, WidgetsState } from "../../hooks";
import { LuFileScan, LuFolderOpen, LuRepeat } from "react-icons/lu";
import { toast } from "sonner";
import { emitRenderToCanvas } from "../../../core/events";
import { invokeOpenInWidgetsDir } from "../../../core/commands";

interface Props {
  widgets: WidgetsState;
  rescan: RescanCallback;
}

const GlobalActions = memo(({ widgets, rescan }: Props) => {
  const refreshAction = useCallback(() => {
    emitRenderToCanvas(widgets.map(({ id, settings }) => ({ id, settings })))
      .then(() => {
        toast.success(`Refreshed ${widgets.length} widgets.`);
      })
      .catch(console.error);
  }, [widgets]);

  const rescanAction = useCallback(() => {
    rescan()
      .then(({ numAdded, numRemoved, numUpdated }) => {
        toast.success(
          `${numAdded} added, ${numRemoved} removed, ${numUpdated} refreshed.`,
        );
      })
      .catch(console.error);
  }, [rescan]);

  const openAction = useCallback(() => {
    invokeOpenInWidgetsDir({ components: [] }).catch(console.error);
  }, []);

  return (
    <Flex gap="6" align="center" justify="center" pb="2" pr="4">
      <IconButton
        title="Refresh current widgets"
        size="1"
        variant="ghost"
        onClick={refreshAction}
        disabled={widgets.length === 0}
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
