import { LuFileScan, LuFolderOpen, LuRepeat } from "react-icons/lu";
import { invokeOpenInWidgetsDir } from "../../core/commands";
import { Flex, ScrollArea, Tabs } from "@radix-ui/themes";
import { toast } from "sonner";
import { FloatButton, WidgetContent, WidgetTrigger } from "../components";
import { emitRenderToCanvas } from "../../core/events";
import { RescanCallback, WidgetsDispatch, WidgetsState } from "../hooks";

interface Props {
  widgets: WidgetsState;
  widgetsDispatch: WidgetsDispatch;
  rescan: RescanCallback;
}

/**
 * The widgets tab in the manager.
 *
 * This tab is rendered as a vertical tab list along with {@link FloatButton}s in the
 * bottom right corner. It contains the triggers {@link WidgetTrigger} and the contents
 * {@link WidgetContent} for each widget in the collection.
 */
export default ({ widgets, widgetsDispatch, rescan }: Props) => {
  const widgetsArray = Object.entries(widgets);

  const rerenderAction = async () => {
    await Promise.all(
      widgetsArray.map(([id, { settings }]) =>
        emitRenderToCanvas({ id, settings }),
      ),
    );
    toast.success(`Re-rendered ${widgetsArray.length} widgets.`);
  };

  const rescanAction = async () => {
    const { numAdded, numRemoved, numUpdated } = await rescan();
    toast.success(
      `${numAdded} added, ${numRemoved} removed, ${numUpdated} updated.`,
    );
  };

  return (
    <>
      <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
        <Flex gap="3" height="100%">
          {widgetsArray.length > 0 && (
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
                  {widgetsArray.map(([id, { config }], index) => (
                    <WidgetTrigger key={id} index={index} config={config} />
                  ))}
                </Flex>
              </ScrollArea>
            </Tabs.List>
          )}
          {widgetsArray.map(([id, { config, settings }], index) => (
            <WidgetContent
              key={id}
              index={index}
              id={id}
              config={config}
              settings={settings}
              widgetsDispatch={widgetsDispatch}
            />
          ))}
        </Flex>
      </Tabs.Root>
      <FloatButton
        order={3}
        icon={<LuRepeat />}
        tooltip="Re-render all widgets"
        onClick={rerenderAction}
        disabled={widgetsArray.length === 0}
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
        onClick={() => invokeOpenInWidgetsDir({ components: [] })}
      />
    </>
  );
};
