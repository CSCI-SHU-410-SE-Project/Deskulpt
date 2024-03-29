import { Box, Button, IconButton, List, ListItem, ListItemText } from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";

import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { BundlerOutputPayload, WidgetConfig } from "./types";

function WidgetInfoTab(props: {
  widgetConfig: WidgetConfig;
  refreshWidget: () => void;
}) {
  const { widgetConfig, refreshWidget } = props;

  return (
    <ListItem
      secondaryAction={
        <IconButton onClick={refreshWidget}>
          <RefreshIcon />
        </IconButton>
      }
    >
      <ListItemText primary={widgetConfig.deskulpt_conf.name} />
    </ListItem>
  );
}

function App() {
  const [widgetConfigs, setWidgetConfigs] = useState<Record<string, WidgetConfig>>({});

  /**
   * Call backend command to bundle the widget and emit a "render-widget" event.
   */
  async function refreshWidget(widgetId: string) {
    const bundlerOutputPayload: BundlerOutputPayload = await invoke("bundle_widget", {
      widgetId,
    });
    // Emit the render-widget event that will be listened to by the canvas window
    await emit("render-widget", { widgetId, bundlerOutputPayload });
  }

  /**
   * Call backend command to refresh widget collection by re-scanning the widget base
   * directory, then refresh each widget as in `refreshWidget`.
   */
  async function refreshAllWidgets() {
    // Update widget configurations
    const widgetConfigs: Record<string, WidgetConfig> = await invoke(
      "refresh_widget_collection",
    );
    setWidgetConfigs(widgetConfigs);

    // Refresh all widgets asynchronously in parallel
    await Promise.all(
      Object.keys(widgetConfigs).map((widgetId) => refreshWidget(widgetId)),
    );
  }

  useEffect(() => {
    refreshAllWidgets().catch(console.error);
  }, []);

  return (
    <Box>
      <List>
        {Object.entries(widgetConfigs).map(([widgetId, widgetConfig]) => (
          <WidgetInfoTab
            key={widgetId}
            widgetConfig={widgetConfig}
            refreshWidget={() => refreshWidget(widgetId)}
          />
        ))}
      </List>
      <Button variant="outlined" onClick={refreshAllWidgets}>
        Refresh
      </Button>
    </Box>
  );
}

export default App;
