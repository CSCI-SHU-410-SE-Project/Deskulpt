import { Box, Button, IconButton, List, ListItem, ListItemText } from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";

import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { WidgetConfig } from "./types";
import { WidgetState, initWidgetState } from "./utils/widgetState";

export default function App() {
  const [widgetConfigs, setWidgetConfigs] = useState<Record<string, WidgetConfig>>({});

  // Internal widget states managed by the app
  // For now, we only keep track of the widget's APIs blob URL
  const widgetsState: Record<string, WidgetState> = {};

  /**
   * Open the widget base directory in the file explorer of the OS.
   */
  async function openWidgetBase() {
    await invoke("open_widget_base").catch(console.error);
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
    return await invoke<Record<string, WidgetConfig>>("refresh_widget_collection")
      .then((output) => {
        setWidgetConfigs(output);
        return output;
      })
      .catch((error) => {
        console.error(error);
        return null;
      });
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
    if (!(widgetId in widgetsState)) {
      console.log(`Initializing widget state for ${widgetId}`);
      widgetsState[widgetId] = await initWidgetState(widgetId);
    }
    console.log(
      `Rendering widget ${widgetId} with apis blob url ${widgetsState[widgetId].widgetApisBlobUrl}`,
    );
    await invoke<string>("bundle_widget", {
      widgetId: widgetId,
      widgetApisUrl: widgetsState[widgetId].widgetApisBlobUrl,
    })
      .then(async (bundlerOutput) => {
        await emit("render-widget", { widgetId, bundlerOutput, success: true });
      })
      .catch(async (error: string) => {
        console.log(typeof error);
        console.log({ error });
        await emit("render-widget", { widgetId, bundlerOutput: error, success: false });
      });
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
              <ListItemText primary={widgetConfig.deskulpt.name} secondary={widgetId} />
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
  );
}
