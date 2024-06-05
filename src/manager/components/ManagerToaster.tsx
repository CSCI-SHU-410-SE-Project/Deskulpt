import { Toaster } from "sonner";
import { ThemeAppearance } from "../../types/backend";

interface ManagerToasterProps {
  themeAppearance: ThemeAppearance;
}

export default function ManagerToaster({ themeAppearance }: ManagerToasterProps) {
  return (
    <Toaster
      position="bottom-center"
      theme={themeAppearance}
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
}
