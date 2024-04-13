import { Box, Button, IconButton, List, ListItem, ListItemText } from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";

import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { CommandOut, WidgetConfig } from "./types";
import WidgetManager from "./WidgetManager/WidgetManager.tsx";

export default function App() {
  const [widgetConfigs, setWidgetConfigs] = useState<Record<string, WidgetConfig>>({});

  /**
   * Open the widget base directory in the file explorer of the OS.
   */
  async function openWidgetBase() {
    const output: CommandOut<null> = await invoke("open_widget_base");
    if ("failure" in output) {
      console.error(output.failure);
    }
  }

  /**
   * Refresh the state of `widgetConfigs`.
   *
   * The reason why this function returns the updated state is that in some cases we
   * need to access the updated state before React actually updates the component.
   *
   * @returns The updated state of `widgetConfigs` if the operation is successful or
   * `null` otherwise.
   */
  async function refreshWidgetCollection() {
    const output: CommandOut<Record<string, WidgetConfig>> = await invoke(
      "refresh_widget_collection",
    );
    if ("success" in output) {
      setWidgetConfigs(output.success);
      return output.success;
    } else {
      console.error(output.failure);
      return null;
    }
  }

  /**
   * Render a widget.
   *
   * In essence, this simply calls the backend command to bundle the widget and emit a
   * corresponding "render-widget" event. The canvas will listen to this event and
   * manage the actual rendering.
   *
   * @param widgetId The ID of the widget to render.
   */
  async function renderWidget(widgetId: string) {
    const bundlerOutput: CommandOut<string> = await invoke("bundle_widget", {
      widgetId,
    });
    await emit("render-widget", { widgetId, bundlerOutput });
  }

  /**
   * Render a collection of widgets asynchronously in parallel.
   *
   * @param configs The collection of widget configurations to render.
   */
  async function renderWidgets(configs: Record<string, WidgetConfig>) {
    await Promise.all(Object.keys(configs).map((widgetId) => renderWidget(widgetId)));
  }

  useEffect(() => {
    // Fetch the widget collection and render all on mount
    refreshWidgetCollection()
      .then(async (configs) => {
        if (configs !== null) {
          await renderWidgets(configs);
        }
      })
      .catch(console.error);
  }, []);

  return (
    <>
      <Box>
        <List>
          {Object.entries(widgetConfigs)
            .sort()
            .map(([widgetId, widgetConfig]) => (
              <ListItem
                key={widgetId}
                secondaryAction={
                  <IconButton onClick={() => renderWidget(widgetId)}>
                    <RefreshIcon />
                  </IconButton>
                }
              >
                <ListItemText
                  primary={widgetConfig.deskulpt.name}
                  secondary={widgetId}
                />
              </ListItem>
            ))}
        </List>
        <Button variant="outlined" onClick={refreshWidgetCollection}>
          Rescan
        </Button>
        <Button variant="outlined" onClick={() => renderWidgets(widgetConfigs)}>
          Render All
        </Button>
        <Button variant="outlined" onClick={openWidgetBase}>
          Open Widget Base Directory
        </Button>
      </Box>
      <WidgetManager />
    </>
  );
}
