import WidgetContainer from "./components/WidgetContainer";
import { Toaster } from "sonner";
import { Theme as RadixTheme } from "@radix-ui/themes";
import { useShallow } from "zustand/shallow";
import {
  useRemoveWidgetsListener,
  useRenderWidgetsListener,
  useShowToastListener,
  useTheme,
  useUpdateSettingsListener,
  useWidgetsStore,
} from "./hooks";

const App = () => {
  const theme = useTheme();
  const ids = useWidgetsStore(
    useShallow((state) => Object.keys(state.widgets)),
  );

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
      {ids.map((id) => (
        <WidgetContainer key={id} id={id} />
      ))}
    </RadixTheme>
  );
};

export default App;
