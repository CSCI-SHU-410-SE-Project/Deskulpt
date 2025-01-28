import { Flex, Tabs } from "@radix-ui/themes";
import { WidgetContent } from "../../components";
import { RescanCallback, WidgetsDispatch, WidgetsState } from "../../hooks";
import FloatButtonRerender from "./FloatButtonRerender";
import FloatButtonRescan from "./FloatButtonRescan";
import FloatButtonOpen from "./FloatButtonOpen";
import Triggers from "./Triggers";

interface Props {
  widgets: WidgetsState;
  widgetsDispatch: WidgetsDispatch;
  rescan: RescanCallback;
}

export default ({ widgets, widgetsDispatch, rescan }: Props) => {
  return (
    <>
      <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
        <Flex gap="3" height="100%">
          {widgets.length > 0 && <Triggers widgets={widgets} />}
          {widgets.map(({ id, config, settings }, index) => (
            <WidgetContent
              value={`tab${index}`}
              key={id}
              id={id}
              config={config}
              settings={settings}
              widgetsDispatch={widgetsDispatch}
            />
          ))}
        </Flex>
      </Tabs.Root>
      <FloatButtonRerender widgets={widgets} />
      <FloatButtonRescan rescan={rescan} />
      <FloatButtonOpen />
    </>
  );
};
