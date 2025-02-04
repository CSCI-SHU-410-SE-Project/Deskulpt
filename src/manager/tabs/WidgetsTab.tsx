import { LuFileScan, LuFolderOpen, LuRepeat } from "react-icons/lu";
import { commands, events } from "../../core";
import { Flex, ScrollArea, Tabs } from "@radix-ui/themes";
import { toast } from "sonner";
import WidgetTrigger from "../components/WidgetTrigger";
import WidgetContent from "../components/WidgetContent";
import FloatButton from "../components/FloatButton";
import { rescan, useWidgetsStore } from "../hooks";

const WidgetsTab = () => {
  const widgets = useWidgetsStore((state) => state.widgets);

  const rerenderAction = async () => {
    await events.renderWidgets.toCanvas(
      widgets.map(({ id, settings }) => ({ id, settings })),
    );
    toast.success(`Re-rendered ${widgets.length} widgets.`);
  };

  const rescanAction = async () => {
    const count = await rescan();
    if (count === 0) {
      toast.success("Rescanned base directory.");
    } else {
      toast.success(
        `Rescanned base directory and re-rendered ${count} widgets.`,
      );
    }
  };

  return (
    <>
      <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
        <Flex gap="3" height="100%">
          {widgets.length > 0 && (
            <Tabs.List
              css={{
                flex: 1,
                height: "100%",
                // Move the shadow of the tab list from bottom to right
                boxShadow: "inset -1px 0 0 0 var(--gray-a5)",
              }}
            >
              <ScrollArea scrollbars="vertical" asChild>
                <Flex direction="column">
                  {widgets.map(({ id, config }, index) => (
                    <WidgetTrigger key={id} index={index} config={config} />
                  ))}
                </Flex>
              </ScrollArea>
            </Tabs.List>
          )}
          {widgets.map(({ id, config, settings }, index) => (
            <WidgetContent
              key={id}
              index={index}
              id={id}
              config={config}
              settings={settings}
            />
          ))}
        </Flex>
      </Tabs.Root>
      <FloatButton
        order={3}
        icon={<LuRepeat />}
        tooltip="Re-render all widgets"
        onClick={rerenderAction}
        disabled={widgets.length === 0}
      />
      <FloatButton
        order={2}
        icon={<LuFileScan />}
        tooltip="Rescan widgets"
        onClick={rescanAction}
      />
      <FloatButton
        order={1}
        icon={<LuFolderOpen />}
        tooltip="Open base directory"
        onClick={() => commands.openInWidgetsDir({ components: [] })}
      />
    </>
  );
};

export default WidgetsTab;
