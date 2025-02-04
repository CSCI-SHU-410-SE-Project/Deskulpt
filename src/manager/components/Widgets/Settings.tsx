import { Box } from "@radix-ui/themes";
import { LuRepeat } from "react-icons/lu";
import { events } from "../../../core";
import { toast } from "sonner";
import WidgetContentHeading from "./WidgetContentHeading";
import WidgetContentSettingsList from "./WidgetContentSettingsList";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";

interface SettingsProps {
  id: string;
}

const Settings = memo(({ id }: SettingsProps) => {
  const settings = useWidgetsStore((state) => state.widgets[id].settings);

  return (
    <>
      <WidgetContentHeading
        heading="Settings"
        actionIcon={<LuRepeat />}
        actionText="Re-render"
        action={() =>
          events.renderWidgets
            .toCanvas([{ id }])
            .then(() => toast.success("Re-rendered widget."))
        }
      />
      <Box px="2" css={{ flex: 4 }}>
        <WidgetContentSettingsList id={id} settings={settings} />
      </Box>
    </>
  );
});

export default Settings;
