import { Box, Button, IconButton, List, ListItem, ListItemText } from "@mui/material";
import RefreshIcon from "@mui/icons-material/Refresh";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { ManagerWidgetState } from "../types";
import { Settings } from "../types";
import { useToggleShortcut } from "../hooks/useToggleShortcut";
import { getNewManagerWidgetStates, renderWidgets } from "./utils";
import { useExitAppListener } from "../hooks/useExitAppListener";

export default function App(props: { initialSettings: Settings }) {
  const { initialSettings } = props;
  const { toggleShortcut, setToggleShortcut } = useToggleShortcut(
    initialSettings.toggleShortcut,
  );
  const [managerWidgetStates, setManagerWidgetStates] = useState<
    Record<string, ManagerWidgetState>
  >({});

  useExitAppListener(toggleShortcut, managerWidgetStates);

  /**
   * Rescan the widget base directory and render newly added widgets.
   *
   * Newly added widgets are those that exist in the new states but does not exist in
   * the previous states.
   */
  async function rescanAndRender() {
    const newManagerWidgetStates = await getNewManagerWidgetStates(
      managerWidgetStates,
      initialSettings.widgetSettings,
    );
    const addedStates = Object.fromEntries(
      Object.entries(newManagerWidgetStates).filter(
        ([widgetId]) => !(widgetId in managerWidgetStates),
      ),
    );
    setManagerWidgetStates(newManagerWidgetStates); // Direct replacement
    await renderWidgets(addedStates);
  }

  useEffect(() => {
    // The rescan is guaranteed to succeed because it triggers a command in the backend;
    // the rendering, however, mail fail due to the canvas not being ready to receive
    // the rendering events; this should be rare with a 2-second timeout
    const timer = setTimeout(() => {
      rescanAndRender().catch(console.error);
    }, 2000);

    return () => {
      clearTimeout(timer);
    };
  }, []);

  return (
    <Box>
      <List>
        {Object.entries(managerWidgetStates)
          .sort()
          .map(([widgetId, state]) => (
            <ListItem
              key={widgetId}
              secondaryAction={
                <IconButton onClick={() => renderWidgets({ [widgetId]: state })}>
                  <RefreshIcon />
                </IconButton>
              }
            >
              <ListItemText
                primary={
                  "Ok" in state.config ? state.config.Ok.deskulptConf.name : "???"
                }
                secondary={widgetId}
              />
            </ListItem>
          ))}
      </List>
      <Button variant="outlined" onClick={rescanAndRender}>
        Rescan
      </Button>
      <Button variant="outlined" onClick={() => renderWidgets(managerWidgetStates)}>
        Render All
      </Button>
      <Button
        variant="outlined"
        onClick={() => invoke("open_widget_base").catch(console.error)}
      >
        View Widgets
      </Button>
      <Button variant="outlined" onClick={() => setToggleShortcut("CmdorCtrl+Shift+G")}>
        Change Shortcut
      </Button>
      <Box>Current shortcut: {toggleShortcut}</Box>
    </Box>
  );
}
