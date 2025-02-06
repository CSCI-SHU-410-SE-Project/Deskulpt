import { Badge, Button, Flex } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { commands, events } from "../../../core";
import { toast } from "sonner";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { useWidgetsStore } from "../../hooks";
import { WidgetConfigType } from "../../../types";

interface HeaderProps {
  id: string;
}

const Header = memo(({ id }: HeaderProps) => {
  const type = useWidgetsStore((state) => state.widgets[id].config.type);

  const refreshAction = useCallback(() => {
    events.renderWidgets.toCanvas([{ id }]).then(() => {
      toast.success("Widget refreshed.");
    });
  }, [id]);

  const openAction = useCallback(() => {
    commands.openWidget({ id }).catch(console.error);
  }, [id]);

  return (
    <Flex align="center" justify="between">
      <Badge color={type === WidgetConfigType.VALID ? "gray" : "red"}>
        ID: {id}
      </Badge>
      <Flex align="center" gap="2">
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

export default Header;
