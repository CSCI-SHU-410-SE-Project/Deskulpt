import { Toaster } from "sonner";
import { Theme } from "../../types/backend";

interface ManagerToasterProps {
  /** The theme. */
  theme: Theme;
}

/**
 * The toaster component for the manager window.
 *
 * This is styled on top of [`Toaster`](https://sonner.emilkowal.ski/toaster), rendered
 * in the bottom center of the manager window.
 */
const ManagerToaster = ({ theme }: ManagerToasterProps) => {
  return (
    <Toaster
      position="bottom-center"
      theme={theme}
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

export default ManagerToaster;
