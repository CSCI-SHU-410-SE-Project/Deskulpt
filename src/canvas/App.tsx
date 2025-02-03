import { Toaster } from "sonner";
import { Box, Theme as RadixTheme } from "@radix-ui/themes";
import {
  useBatchRemoveListener,
  useRender,
  useShowToastListener,
  useTheme,
  useUpdateSettingsListener,
  useWidgetsStore,
} from "./hooks";
import { RenderingScreen, WidgetContainer } from "./components";
import { css } from "@emotion/react";
import { useShallow } from "zustand/shallow";

const styles = {
  root: css({ height: "100vh" }),
};

export default () => {
  const theme = useTheme();
  const ids = useWidgetsStore(
    useShallow((state) => Object.keys(state.widgets)),
  );

  const isRendering = useRender();

  useBatchRemoveListener();
  useShowToastListener();
  useUpdateSettingsListener();

  return (
    <RadixTheme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      hasBackground={false}
      css={styles.root}
    >
      {isRendering && <RenderingScreen />}
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
      <Box>
        {ids.map((id) => (
          <WidgetContainer key={id} id={id} />
        ))}
      </Box>
    </RadixTheme>
  );
};
