import { Box, Button, IconButton, List, ListItem, ListItemText } from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { WidgetConfig, WidgetState } from "./types";
import { createWidgetApisBlob } from "./utils";

export default function App() {
  const [widgetStates, setWidgetStates] = useState<Record<string, WidgetState>>({});

  /**
   * Open the widget base directory in the file explorer of the OS.
   */
  async function openWidgetBase() {
    await invoke("open_widget_base").catch(console.error);
  }

  /**
   * Rescan the widget base directory.
   *
   * This function fetches the widget collection from the backend and wraps them with
   * states in the frontend. It updates `widgetStates` and also returns the updated
   * widget states (because in some cases we need to access the updated states before
   * React actually re-renders the component). It returns `null` if the operation fails.
   */
  async function rescanWidgetBase() {
    return await invoke<Record<string, WidgetConfig>>("refresh_widget_collection")
      .then(async (widgetConfigs) => {
        const cleanupRemovedWidgets = async (removedIds: string[]) => {
          // Notify the canvas to cleanup resourced allocated for removed widgets
          await emit("remove-widgets", { removedIds });

          // Revoke the API blob URLs of removed widgets for optimal performance and
          // memory usage as they will not be used anymore; even if the same widget ID
          // appears some time later, the blob URL will be recreated rather than reused
          removedIds.forEach((id) => URL.revokeObjectURL(widgetStates[id].apisBlobUrl));
        };

        // If a widget exists in the previous states but does not exist in the new
        // scanning result, we consider it as removed and perform cleanup
        const removedIds = Object.keys(widgetStates).filter(
          (id) => !(id in widgetConfigs),
        );
        if (removedIds.length > 0) {
          await cleanupRemovedWidgets(removedIds);
        }

        const createWidgetState = async (
          widgetId: string,
          config: WidgetConfig,
        ): Promise<[string, WidgetState]> => {
          // Reuse the blob URL of widget APIs if the widget state previously exists;
          // create a new one otherwise
          const apisBlobUrl =
            widgetId in widgetStates
              ? widgetStates[widgetId].apisBlobUrl
              : URL.createObjectURL(await createWidgetApisBlob(widgetId));
          return [widgetId, { config, apisBlobUrl }];
        };

        const newWidgetStates = Object.fromEntries(
          await Promise.all(
            Object.entries(widgetConfigs).map(([widgetId, config]) =>
              createWidgetState(widgetId, config),
            ),
          ),
        );
        setWidgetStates(newWidgetStates);
        return newWidgetStates;
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
   */
  async function renderWidget(widgetId: string, apisBlobUrl: string) {
    await invoke<string>("bundle_widget", { widgetId, apisBlobUrl })
      .then(async (bundlerOutput) => {
        await emit("render-widget", { widgetId, bundlerOutput, success: true });
      })
      .catch(async (error: string) => {
        await emit("render-widget", { widgetId, bundlerOutput: error, success: false });
      });
  }

  /**
   * Render a collection of widgets asynchronously in parallel.
   */
  async function renderWidgets(states: Record<string, WidgetState>) {
    await Promise.all(
      Object.entries(states).map(([widgetId, widgetState]) =>
        renderWidget(widgetId, widgetState.apisBlobUrl),
      ),
    );
  }

  /**
   * Rescan the widget base directory and render newly added widgets.
   *
   * Newly added widgets are those that exist in the new scanning result but does not
   * exist in the previous states.
   */
  async function rescanAndRender() {
    const states = await rescanWidgetBase();
    if (states !== null) {
      const addedStates = Object.fromEntries(
        Object.entries(states).filter(([widgetId]) => !(widgetId in widgetStates)),
      );
      await renderWidgets(addedStates);
    }
  }

  useEffect(() => {
    rescanAndRender().catch(console.error);
  }, []);

  return (
    <Box>
      <List>
        {Object.entries(widgetStates)
          .sort()
          .map(([widgetId, widgetState]) => (
            <ListItem
              key={widgetId}
              secondaryAction={
                <IconButton
                  onClick={() =>
                    renderWidget(widgetId, widgetStates[widgetId].apisBlobUrl)
                  }
                >
                  <RefreshIcon />
                </IconButton>
              }
            >
              <ListItemText
                primary={widgetState.config.deskulpt.name}
                secondary={widgetId}
              />
            </ListItem>
          ))}
      </List>
      <Button variant="outlined" onClick={rescanAndRender}>
        Rescan
      </Button>
      <Button variant="outlined" onClick={() => renderWidgets(widgetStates)}>
        Render All
      </Button>
      <Button variant="outlined" onClick={openWidgetBase}>
        View Widgets
      </Button>
    </Box>
  );
}
