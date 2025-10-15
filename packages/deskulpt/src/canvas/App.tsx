import { Toaster } from "sonner";
import { Theme as RadixTheme } from "@radix-ui/themes";
import {
  useRemoveWidgetsListener,
  useRenderWidgetsListener,
  useSettingsStore,
  useShowToastListener,
  useUpdateSettingsListener,
  useWebviewManager,
} from "./hooks";

const App = () => {
  const theme = useSettingsStore((state) => state.theme);

  // Initialize webview manager
  useWebviewManager();

  useRemoveWidgetsListener();
  useRenderWidgetsListener();
  useShowToastListener();
  useUpdateSettingsListener();

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
      {/* Widgets are now rendered in separate webviews managed by useWebviewManager */}
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
        Canvas running with separate webviews per widget
      </div>
    </RadixTheme>
  );
};

export default App;
