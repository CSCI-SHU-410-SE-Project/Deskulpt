import { useEffect, useRef, useState } from "react";
import { Theme as RadixTheme } from "@radix-ui/themes";
import { Toaster } from "sonner";
import { ErrorBoundary, FallbackProps } from "react-error-boundary";
import ErrorDisplay from "./components/ErrorDisplay";
import { stringifyError } from "../utils/stringifyError";
import { commands, events } from "../bindings";
import { WidgetSettings } from "../bindings";
import { FC } from "react";

// Get widget ID from URL parameters
const getWidgetId = () => {
  const urlParams = new URLSearchParams(window.location.search);
  return urlParams.get("id");
};

const BASE_URL = new URL(import.meta.url).origin;
const RAW_APIS_URL = new URL("/gen/raw-apis.js", BASE_URL).href;

interface WidgetProps extends WidgetSettings {
  id: string;
}

interface WidgetState {
  component?: FC<WidgetProps>;
  settings?: WidgetSettings;
  error?: { error: string; message: string };
  apisBlobUrl?: string;
  moduleBlobUrl?: string;
}

const WidgetApp = () => {
  const widgetId = getWidgetId();
  const [widgetState, setWidgetState] = useState<WidgetState>({});
  const hasInited = useRef(false);

  useEffect(() => {
    if (!widgetId) {
      console.error("No widget ID provided in URL parameters");
      return;
    }

    // Listen for render events for this specific widget
    const unlisten = events.renderWidgets.listen(async (event) => {
      if (event.payload.includes(widgetId)) {
        await renderWidget();
      }
    });

    const renderWidget = async () => {
      let apisBlobUrl = widgetState.apisBlobUrl;

      // Create APIs blob if not exists
      if (!apisBlobUrl) {
        const apisCode = window.__DESKULPT_CANVAS_INTERNALS__.apisWrapper
          .replaceAll("__DESKULPT_WIDGET_ID__", widgetId)
          .replaceAll("__RAW_APIS_URL__", RAW_APIS_URL);
        const apisBlob = new Blob([apisCode], {
          type: "application/javascript",
        });
        apisBlobUrl = URL.createObjectURL(apisBlob);
      }

      // Clean up previous module blob
      if (widgetState.moduleBlobUrl) {
        URL.revokeObjectURL(widgetState.moduleBlobUrl);
      }

      let code;
      try {
        code = await commands.core.bundleWidget(widgetId);
      } catch (error) {
        setWidgetState((prev) => ({
          ...prev,
          error: {
            error: "Error bundling the widget",
            message: stringifyError(error),
          },
          apisBlobUrl,
        }));
        return;
      }

      let moduleCode = code
        .replaceAll("__DESKULPT_BASE_URL__", BASE_URL)
        .replaceAll("__DESKULPT_APIS_BLOB_URL__", apisBlobUrl);
      const moduleBlob = new Blob([moduleCode], {
        type: "application/javascript",
      });
      const moduleBlobUrl = URL.createObjectURL(moduleBlob);

      let module;
      try {
        module = await import(/* @vite-ignore */ moduleBlobUrl);
        if (module.default === undefined) {
          throw new Error("Missing default export");
        }
      } catch (error) {
        URL.revokeObjectURL(moduleBlobUrl);
        setWidgetState((prev) => ({
          ...prev,
          error: {
            error: "Error importing the widget module",
            message: stringifyError(error),
          },
          apisBlobUrl,
        }));
        return;
      }

      setWidgetState((prev) => ({
        ...prev,
        component: module.default,
        moduleBlobUrl,
        apisBlobUrl,
        error: undefined,
      }));
    };

    // Set the widget as ready to render only once
    if (!hasInited.current) {
      commands.core
        .setRenderReady()
        .then(() => {
          hasInited.current = true;
        })
        .catch(console.error);
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [widgetId, widgetState.apisBlobUrl, widgetState.moduleBlobUrl]);

  // Listen for settings updates
  useEffect(() => {
    if (!widgetId) return;

    const unlisten = events.updateSettings.listen((event) => {
      const newSettings = event.payload.widgets[widgetId];
      if (newSettings) {
        setWidgetState((prev) => ({
          ...prev,
          settings: newSettings,
        }));
      }
    });

    // Get initial settings
    const initialSettings =
      window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.widgets[widgetId];
    if (initialSettings) {
      setWidgetState((prev) => ({
        ...prev,
        settings: initialSettings,
      }));
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [widgetId]);

  // Cleanup blob URLs on unmount
  useEffect(() => {
    return () => {
      if (widgetState.apisBlobUrl) {
        URL.revokeObjectURL(widgetState.apisBlobUrl);
      }
      if (widgetState.moduleBlobUrl) {
        URL.revokeObjectURL(widgetState.moduleBlobUrl);
      }
    };
  }, [widgetState.apisBlobUrl, widgetState.moduleBlobUrl]);

  if (!widgetId) {
    return (
      <RadixTheme
        appearance="light"
        accentColor="indigo"
        grayColor="slate"
        hasBackground={false}
      >
        <ErrorDisplay
          id=""
          error="Invalid widget configuration"
          message="No widget ID provided in URL parameters"
        />
      </RadixTheme>
    );
  }

  if (widgetState.error) {
    return (
      <RadixTheme
        appearance="light"
        accentColor="indigo"
        grayColor="slate"
        hasBackground={false}
      >
        <Toaster
          position="bottom-right"
          gap={6}
          toastOptions={{
            style: {
              color: "var(--gray-12)",
              borderColor: "var(--gray-6)",
              backgroundColor: "var(--gray-2)",
              padding: "var(--space-2) var(--space-4)",
            },
          }}
        />
        <ErrorDisplay
          id={widgetId}
          error={widgetState.error.error}
          message={widgetState.error.message}
        />
      </RadixTheme>
    );
  }

  if (!widgetState.component || !widgetState.settings) {
    return (
      <RadixTheme
        appearance="light"
        accentColor="indigo"
        grayColor="slate"
        hasBackground={false}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            height: "100vh",
            color: "var(--gray-11)",
            fontSize: "14px",
          }}
        >
          Loading widget...
        </div>
      </RadixTheme>
    );
  }

  const { component: Widget, settings } = widgetState;
  const theme = window.__DESKULPT_CANVAS_INTERNALS__.initialSettings.theme;

  return (
    <RadixTheme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      hasBackground={false}
    >
      <Toaster
        position="bottom-right"
        gap={6}
        toastOptions={{
          style: {
            color: "var(--gray-12)",
            borderColor: "var(--gray-6)",
            backgroundColor: "var(--gray-2)",
            padding: "var(--space-2) var(--space-4)",
          },
        }}
      />
      <div style={{ opacity: settings.opacity / 100 }}>
        <ErrorBoundary
          resetKeys={[Widget]}
          fallbackRender={(props: FallbackProps) => (
            <ErrorDisplay
              id={widgetId}
              error="Error in the widget component [React error boundary]"
              message={stringifyError(props.error)}
            />
          )}
        >
          <Widget
            id={widgetId}
            x={settings.x}
            y={settings.y}
            width={settings.width}
            height={settings.height}
            opacity={settings.opacity}
          />
        </ErrorBoundary>
      </div>
    </RadixTheme>
  );
};

export default WidgetApp;
