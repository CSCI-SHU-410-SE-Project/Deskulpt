import { Flex, ScrollArea, Separator, Tabs } from "@radix-ui/themes";
import WidgetTrigger from "./WidgetTrigger";
import WidgetContent from "./WidgetContent";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";
import { useShallow } from "zustand/shallow";
import GlobalActions from "./GlobalActions";

const WidgetsTab = memo(() => {
  const ids = useWidgetsStore(
    useShallow((state) => Object.keys(state.widgets)),
  );

  // const rerenderAction = async () => {
  //   await events.renderWidgets.toCanvas(
  //     widgets.map(({ id, settings }) => ({ id, settings })),
  //   );
  //   toast.success(`Re-rendered ${widgets.length} widgets.`);
  // };

  // const rescanAction = async () => {
  //   const count = await rescan();
  //   if (count === 0) {
  //     toast.success("Rescanned base directory.");
  //   } else {
  //     toast.success(
  //       `Rescanned base directory and re-rendered ${count} widgets.`,
  //     );
  //   }
  // };

  return (
    <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
      <Flex height="100%">
        <Tabs.List css={{ flex: 1, height: "100%", boxShadow: "none" }}>
          <Flex direction="column" width="100%" gap="4">
            <ScrollArea scrollbars="vertical" asChild>
              <Flex direction="column">
                {ids.map((id, index) => (
                  <WidgetTrigger key={id} index={index} id={id} />
                ))}
              </Flex>
            </ScrollArea>
            <Separator size="4" />
            <GlobalActions length={ids.length} />
          </Flex>
        </Tabs.List>
        {ids.map((id, index) => (
          <WidgetContent key={id} index={index} id={id} />
        ))}
      </Flex>
    </Tabs.Root>
  );
});

export default WidgetsTab;
