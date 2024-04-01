import { Box, Button, IconButton, List, ListItem, ListItemText } from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";

import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { CommandOut, WidgetConfig } from "./types";

function WidgetInfoTab(props: {
  widgetConfig: WidgetConfig;
  renderWidget: () => void;
}) {
  const { widgetConfig, renderWidget } = props;

  return (
    <ListItem
      secondaryAction={
        <IconButton onClick={renderWidget}>
          <RefreshIcon />
        </IconButton>
      }
    >
      <ListItemText primary={widgetConfig.deskulpt.name} />
    </ListItem>
  );
}

function App() {
  const [widgetConfigs, setWidgetConfigs] = useState<Record<string, WidgetConfig>>({});

  async function refreshWidgetCollection() {
    const output: CommandOut<Record<string, WidgetConfig>> = await invoke(
      "refresh_widget_collection",
    );
    if ("success" in output) {
      setWidgetConfigs(output.success);
    } else {
      console.error(output.failure);
    }
  }

  async function renderWidget(widgetId: string) {
    const bundlerOutput: CommandOut<string> = await invoke("bundle_widget", {
      widgetId,
    });
    await emit("render-widget", { widgetId, bundlerOutput });
  }

  async function renderAllWidgets() {
    await Promise.all(
      Object.keys(widgetConfigs).map((widgetId) => renderWidget(widgetId)),
    );
  }

  useEffect(() => {
    refreshWidgetCollection().then(renderAllWidgets).catch(console.error);
  }, []);

  return (
    <Box>
      <List>
        {Object.entries(widgetConfigs).map(([widgetId, widgetConfig]) => (
          <WidgetInfoTab
            key={widgetId}
            widgetConfig={widgetConfig}
            renderWidget={() => renderWidget(widgetId)}
          />
        ))}
      </List>
      <Button variant="outlined" onClick={refreshWidgetCollection}>
        Rescan
      </Button>
      <Button variant="outlined" onClick={renderAllWidgets}>
        Render All
      </Button>
    </Box>
  );
}

export default App;
