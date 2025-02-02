import WidgetContainer from "./components/WidgetContainer";
import useRenderWidgetListener from "./hooks/useRenderWidgetListener";
import useRemoveWidgetsListener from "./hooks/useRemoveWidgetsListener";
import useShowToastListener from "./hooks/useShowToastListener";
import { Toaster } from "sonner";
import { Theme as RadixTheme } from "@radix-ui/themes";
import useThemeListener from "./hooks/useThemeListener";
import { useWidgetsStore } from "./hooks/useWidgetsStore";
import { useShallow } from "zustand/shallow";

const App = () => {
  const theme = useThemeListener();
  const ids = useWidgetsStore(
    useShallow((state) => Object.keys(state.widgets)),
  );

  useShowToastListener();
  useRenderWidgetListener();
  useRemoveWidgetsListener();

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
