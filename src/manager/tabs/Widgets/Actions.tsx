import { Badge, Button, Flex } from "@radix-ui/themes";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { toast } from "sonner";
import { emitRenderToCanvas } from "../../../core/events";
import { invokeOpenInWidgetsDir } from "../../../core/commands";
import { memo, useCallback } from "react";

interface Props {
  id: string;
  isValid: boolean;
  dir: string;
}

const Actions = memo(({ id, isValid, dir }: Props) => {
  const refreshAction = useCallback(() => {
    emitRenderToCanvas([{ id }]).then(() => toast.success("Widget refreshed."));
  }, [id]);

  const openAction = useCallback(() => {
    invokeOpenInWidgetsDir({ components: [dir] });
  }, [dir]);

  return (
    <Flex align="center" justify="between">
      <Badge color={isValid ? "gray" : "red"}>ID: {id}</Badge>
      <Flex gap="2">
        <Button
          title="Refresh this widget"
          size="1"
          variant="surface"
          onClick={refreshAction}
        >
          <LuRepeat /> Refresh
        </Button>
        <Button
          title="Open this widget folder"
          size="1"
          variant="surface"
          onClick={openAction}
        >
          <LuFolderOpen /> Edit
        </Button>
      </Flex>
    </Flex>
  );
});

export default Actions;
