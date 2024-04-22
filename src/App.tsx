import { Box, Button, IconButton, List, ListItem, ListItemText } from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";

import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { WidgetConfig } from "./types";

export default function App() {
  const [widgetConfigs, setWidgetConfigs] = useState<Record<string, WidgetConfig>>({});

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
   * This will also emit a "remove-widgets" event to notify the canvas to remove the
   * widgets that are no longer present in the updated state, if any.
   *
   * @returns The updated state of `widgetConfigs` if the operation is successful or
   * `null` otherwise.
   */
  async function refreshWidgetCollection() {
    return await invoke<Record<string, WidgetConfig>>("refresh_widget_collection")
      .then(async (output) => {
        // Check for removed widgets and notify the canvas if any
        const removedIds = Object.keys(widgetConfigs).filter((id) => !(id in output));
        if (removedIds.length > 0) {
          await emit("remove-widgets", { widgetIds: removedIds });
        }

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
    await invoke<string>("bundle_widget", { widgetId })
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
   * @param widgetIds The collection of widget IDs to render.
   */
  async function renderWidgets(widgetIds: string[]) {
    await Promise.all(widgetIds.map((widgetId) => renderWidget(widgetId)));
  }

  /**
   * Rescan widget collection and render newly added widgets.
   */
  async function rescan() {
    await refreshWidgetCollection()
      .then(async (configs) => {
        if (configs !== null) {
          const newIds = Object.keys(configs).filter((id) => !(id in widgetConfigs));
          await renderWidgets(newIds);
        }
      })
      .catch(console.error);
  }

  useEffect(() => {
    rescan().catch(console.error);
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
      <Button variant="outlined" onClick={rescan}>
        Rescan
      </Button>
      <Button
        variant="outlined"
        onClick={() => renderWidgets(Object.keys(widgetConfigs))}
      >
        Render All
      </Button>
      <Button variant="outlined" onClick={openWidgetBase}>
        View Widgets
      </Button>
    </Box>
  );
}
