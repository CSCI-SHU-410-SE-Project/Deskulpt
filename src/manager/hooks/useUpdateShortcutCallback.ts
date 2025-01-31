import { useCallback } from "react";
import { Shortcuts } from "../../types";
import { AppSettingsActionType, AppSettingsDispatch } from "./useAppSettings";
import { invokeUpdateShortcut } from "../../core/commands";
import { toast } from "sonner";

export type UpdateShortcutCallback = (
  key: keyof Shortcuts,
  from: string | null,
  to: string | null,
) => Promise<void>;

export function useUpdateShortcutCallback(
  appSettingsDispatch: AppSettingsDispatch,
) {
  return useCallback(
    (key: keyof Shortcuts, from: string | null, to: string | null) =>
      invokeUpdateShortcut({ key, from, to })
        .then(() => {
          appSettingsDispatch({
            type: AppSettingsActionType.SET_SHORTCUT,
            payload: { key, shortcut: to },
          });
          toast.success(
            to === null ? "Shortcut disabled." : `Shortcut updated: ${to}`,
          );
        })
        .catch((error) => {
          console.error(error);
          toast.error("Failed to update shortcut.");
        }),
    [appSettingsDispatch],
  );
}
