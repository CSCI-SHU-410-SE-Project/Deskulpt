import { Flex, ScrollArea, Separator, Tabs } from "@radix-ui/themes";
import WidgetTrigger from "./WidgetTrigger";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";
import { useShallow } from "zustand/shallow";
import GlobalActions from "./GlobalActions";
import Config from "./Config";
import Settings from "./Settings";

const WidgetsTab = memo(() => {
  const ids = useWidgetsStore(
    useShallow((state) => Object.keys(state.widgets)),
  );

  return (
    <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
      <Flex height="100%">
        <Tabs.List css={{ flex: 1, height: "100%", boxShadow: "none" }}>
          <Flex direction="column" width="100%" gap="4">
            <ScrollArea scrollbars="vertical" asChild>
              <Flex direction="column">
                {ids.map((id, index) => (
                  <WidgetTrigger key={id} id={id} value={`tab${index}`} />
                ))}
              </Flex>
            </ScrollArea>
            <Separator size="4" />
            <GlobalActions length={ids.length} />
          </Flex>
        </Tabs.List>
        {ids.map((id, index) => (
          <Tabs.Content key={id} value={`tab${index}`} asChild>
            <Flex
              height="100%"
              direction="column"
              gap="4"
              pl="2"
              css={{ flex: 3, boxShadow: "inset 1px 0 0 0 var(--gray-a5)" }}
            >
              <Config id={id} />
              <Separator size="4" />
              <Settings id={id} />
            </Flex>
          </Tabs.Content>
        ))}
      </Flex>
    </Tabs.Root>
  );
});

export default WidgetsTab;
