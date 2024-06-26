import { Dispatch, SetStateAction } from "react";
import { LuFileScan, LuFolderOpen, LuRepeat } from "react-icons/lu";
import { invokeOpenWidgetResource } from "../../commands";
import { Flex, ScrollArea, Tabs } from "@radix-ui/themes";
import { toast } from "sonner";
import { ManagerWidgetState } from "../../types/frontend";
import { IdMap } from "../../types/backend";
import WidgetTrigger from "../components/WidgetTrigger";
import WidgetContent from "../components/WidgetContent";
import FloatButton from "../components/FloatButton";
import { emitRenderWidgetToCanvas } from "../../events";

export interface WidgetsTabProps {
  /** The manager widget states. */
  managerWidgetStates: IdMap<ManagerWidgetState>;
  /** Setter for the manager widget states. */
  setManagerWidgetStates: Dispatch<SetStateAction<IdMap<ManagerWidgetState>>>;
  /** The function for refreshing the widget collection. */
  rescanAndRender: () => Promise<number>;
}

/**
 * The widgets tab in the manager.
 *
 * This tab is rendered as a vertical tab list along with {@link FloatButton}s in the
 * bottom right corner. It contains the triggers {@link WidgetTrigger} and the contents
 * {@link WidgetContent} for each widget in the collection.
 */
export default function WidgetsTab({
  managerWidgetStates,
  setManagerWidgetStates,
  rescanAndRender,
}: WidgetsTabProps) {
  const rerenderAction = async () => {
    const managerWidgetStatesArray = Object.entries(managerWidgetStates);
    await Promise.all(
      managerWidgetStatesArray.map(([widgetId, { setting }]) =>
        emitRenderWidgetToCanvas({ widgetId, setting, bundle: true }),
      ),
    );
    toast.success(`Re-rendered ${managerWidgetStatesArray.length} widgets.`);
  };

  const rescanAction = async () => {
    const count = await rescanAndRender();
    if (count === 0) {
      toast.success("Rescanned base directory.");
    } else {
      toast.success(
        `Rescanned base directory and rendered ${count} newly added widgets.`,
      );
    }
  };

  return (
    <>
      <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
        <Flex gap="3" height="100%">
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
                {Object.entries(managerWidgetStates).map(
                  ([widgetId, { config }], index) => (
                    <WidgetTrigger key={widgetId} index={index} config={config} />
                  ),
                )}
              </Flex>
            </ScrollArea>
          </Tabs.List>
          {Object.entries(managerWidgetStates).map(
            ([widgetId, { config, setting }], index) => (
              <WidgetContent
                key={widgetId}
                index={index}
                widgetId={widgetId}
                config={config}
                setting={setting}
                setManagerWidgetStates={setManagerWidgetStates}
              />
            ),
          )}
        </Flex>
      </Tabs.Root>
      <FloatButton
        order={3}
        icon={<LuRepeat />}
        tooltip="Re-render all widgets"
        onClick={rerenderAction}
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
        onClick={() => invokeOpenWidgetResource(null, null)}
      />
    </>
  );
}
