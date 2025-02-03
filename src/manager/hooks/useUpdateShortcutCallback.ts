import { useCallback } from "react";
import { Shortcuts } from "../../types";
import { AppSettingsActionType, AppSettingsDispatch } from "./useAppSettings";
import { invokeUpdateShortcut } from "../../core/commands";
import { toast } from "sonner";

export type UpdateShortcutCallback = (
  key: keyof Shortcuts,
  oldShortcut: string | null,
  newShortcut: string | null,
) => Promise<void>;

export function useUpdateShortcutCallback(
  appSettingsDispatch: AppSettingsDispatch,
) {
  return useCallback(
    (
      key: keyof Shortcuts,
      oldShortcut: string | null,
      newShortcut: string | null,
    ) =>
      invokeUpdateShortcut({ key, oldShortcut, newShortcut })
        .then(() => {
          appSettingsDispatch({
            type: AppSettingsActionType.SET_SHORTCUT,
            payload: { key, shortcut: newShortcut },
          });
          toast.success(
            newShortcut === null
              ? "Shortcut disabled."
              : `Shortcut updated: ${newShortcut}`,
          );
        })
        .catch((error) => {
          console.error(error);
          toast.error("Failed to update shortcut.");
        }),
    [appSettingsDispatch],
  );
}
