import { Toaster } from "sonner";
import { Appearance } from "../../types/backend";

interface Props {
  /** The theme appearance. */
  appearance: Appearance;
}

/**
 * The toaster component for the manager window.
 *
 * This is styled on top of [`Toaster`](https://sonner.emilkowal.ski/toaster), rendered
 * in the bottom center of the manager window.
 */
export default ({ appearance }: Props) => {
  return (
    <Toaster
      position="bottom-center"
      theme={appearance}
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
  );
};
