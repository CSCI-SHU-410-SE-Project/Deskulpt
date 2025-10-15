import { useCallback, useEffect } from "react";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
import { useSettingsStore } from "./useSettingsStore";
import { useWidgetsStore } from "./useWidgetsStore";
import { useShallow } from "zustand/shallow";
import { commands } from "../../bindings";

export function useWebviewManager() {
  const settingsStore = useSettingsStore();
  const widgetIds = useWidgetsStore(useShallow((state) => Object.keys(state)));

  const createWebview = useCallback(
    async (id: string) => {
      const settings = settingsStore.widgets[id];
      if (!settings) return null;

      try {
        // Check if webview already exists
        const existingWebview = await WebviewWindow.getByLabel(`widget-${id}`);
        if (existingWebview) {
          return existingWebview;
        }

        // Create new webview
        console.log("Creating webview for widget", id);
        await commands.core.createWidgetWindow(
          id,
          settings.x,
          settings.y,
          settings.width,
          settings.height,
        );
        console.log("Webview created for widget", id);
        const webview = await WebviewWindow.getByLabel(`widget-${id}`);
        console.log("Webview instance:", webview);

        return webview;
      } catch (error) {
        console.error(`Failed to create webview for widget ${id}:`, error);
        return null;
      }
    },
    [settingsStore.widgets],
  );

  const updateWebviewPosition = useCallback(
    async (id: string, x: number, y: number) => {
      const webview = await WebviewWindow.getByLabel(`widget-${id}`);
      if (webview) {
        try {
          await webview.setPosition(new LogicalPosition(x, y));
        } catch (error) {
          console.error(`Failed to update position for widget ${id}:`, error);
        }
      }
    },
    [],
  );

  const updateWebviewSize = useCallback(
    async (id: string, width: number, height: number) => {
      const webview = await WebviewWindow.getByLabel(`widget-${id}`);
      if (webview) {
        try {
          await webview.setSize(new LogicalSize(width, height));
        } catch (error) {
          console.error(`Failed to update size for widget ${id}:`, error);
        }
      }
    },
    [],
  );

  const destroyWebview = useCallback(async (id: string) => {
    const webview = await WebviewWindow.getByLabel(`widget-${id}`);
    if (webview) {
      try {
        await webview.close();
      } catch (error) {
        console.error(`Failed to destroy webview for widget ${id}:`, error);
      }
    }
  }, []);

  // Create webviews for new widgets
  useEffect(() => {
    const promises = widgetIds.map(async (id) => {
      let existingWebview = await WebviewWindow.getByLabel(`widget-${id}`);
      if (!existingWebview) {
        existingWebview = await createWebview(id);
      }
      return existingWebview;
    });

    Promise.all(promises).catch(console.error);
  }, [widgetIds, createWebview]);

  // Cleanup webviews for removed widgets
  useEffect(() => {
    const currentIds = new Set(widgetIds);
    WebviewWindow.getAll()
      .then((webviews) =>
        webviews.forEach(async (wv) => {
          if (!wv.label.startsWith("widget-")) return;
          const id = wv.label.slice(7); // Remove "widget-" prefix
          if (!currentIds.has(id)) {
            await destroyWebview(id);
          }
        }),
      )
      .catch(console.error);
  }, [widgetIds, destroyWebview]);

  // Update webview positions and sizes when settings change
  useEffect(() => {
    for (const [id, settings] of Object.entries(settingsStore.widgets)) {
      updateWebviewPosition(id, settings.x, settings.y).catch(console.error);
      updateWebviewSize(id, settings.width, settings.height).catch(
        console.error,
      );
    }
  }, [settingsStore.widgets, updateWebviewPosition, updateWebviewSize]);

  return {
    createWebview,
    updateWebviewPosition,
    updateWebviewSize,
    destroyWebview,
  };
}
