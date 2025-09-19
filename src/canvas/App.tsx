import WidgetContainer from "./components/WidgetContainer";
import { Toaster } from "sonner";
import { Theme as RadixTheme } from "@radix-ui/themes";
import { useShallow } from "zustand/shallow";
import { useSettings } from "./hooks/useStores";
import { useWidgetsStore } from "./hooks/useWidgetsStore";
import { useEventListeners } from "./hooks/useEventListeners";

const App = () => {
  const theme = useSettings((state) => state.app.theme);
  const ids = useWidgetsStore(useShallow((state) => Object.keys(state)));

  useEventListeners();

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
