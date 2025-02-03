import { Box, Flex, ScrollArea, Separator, Tabs } from "@radix-ui/themes";
import { RescanCallback, WidgetsDispatch, WidgetsState } from "../../hooks";
import Trigger from "./Trigger";
import { memo } from "react";
import GlobalActions from "./GlobalActions";
import { WidgetConfigType } from "../../../types";
import NoWidget from "./NoWidget";
import Actions from "./Actions";
import Config from "./Config";
import ConfigError from "./ConfigError";
import SettingsLeft from "./SettingsLeft";
import { css } from "@emotion/react";

const styles = {
  tabList: css({ flex: 1, height: "100%", boxShadow: "none" }),
  tabContent: css({ flex: 3, boxShadow: "inset 1px 0 0 0 var(--gray-a5)" }),
};

interface Props {
  widgets: WidgetsState;
  widgetsDispatch: WidgetsDispatch;
  rescan: RescanCallback;
}

const Widgets = memo(({ widgets, widgetsDispatch, rescan }: Props) => {
  return (
    <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
      <Flex gap="0" height="100%">
        <Tabs.List css={styles.tabList}>
          <Flex direction="column" width="100%" gap="4">
            <ScrollArea scrollbars="vertical" asChild>
              <Flex direction="column">
                {widgets.map(({ id, config }, index) => (
                  <Trigger
                    key={id}
                    value={`tab${index}`}
                    isValid={config.type === WidgetConfigType.VALID}
                    dir={config.content.dir}
                  />
                ))}
              </Flex>
            </ScrollArea>
            {widgets.length === 0 && <NoWidget />}
            <Separator size="4" />
            <GlobalActions widgets={widgets} rescan={rescan} />
          </Flex>
        </Tabs.List>
        {widgets.map(({ id, config, settings }, index) => (
          <Tabs.Content key={id} value={`tab${index}`} asChild>
            <Flex
              height="100%"
              direction="column"
              gap="4"
              pl="2"
              css={styles.tabContent}
            >
              <Actions
                id={id}
                isValid={config.type === WidgetConfigType.VALID}
                dir={config.content.dir}
              />
              <Box px="2" height="160px">
                {config.type === WidgetConfigType.VALID ? (
                  <Config
                    name={config.content.name}
                    entry={config.content.entry}
                    dependencies={config.content.dependencies}
                  />
                ) : (
                  <ConfigError error={config.content.error} />
                )}
              </Box>
              <Separator size="4" />
              <Flex px="2" gap="2">
                <SettingsLeft
                  id={id}
                  settings={settings}
                  widgetsDispatch={widgetsDispatch}
                />
                {/* Placeholder for SettingsRight */}
              </Flex>
            </Flex>
          </Tabs.Content>
        ))}
        {/* <Contents widgets={widgets} widgetsDispatch={widgetsDispatch} /> */}
      </Flex>
    </Tabs.Root>
  );
});

export default Widgets;
