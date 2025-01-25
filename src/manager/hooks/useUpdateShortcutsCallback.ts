import { useCallback } from "react";
import { Shortcuts } from "../../types";
import { AppSettingsActionType, AppSettingsDispatch } from "./useAppSettings";
import { invokeUpdateShortcuts } from "../../core/commands";
import { toast } from "sonner";

export type UpdateShortcutsCallback = (
  oldShortcuts: Shortcuts,
  newShortcuts: Shortcuts,
) => Promise<void>;

export function useUpdateShortcutsCallback(
  appSettingsDispatch: AppSettingsDispatch,
) {
  return useCallback(
    (oldShortcuts: Shortcuts, newShortcuts: Shortcuts) =>
      invokeUpdateShortcuts({ oldShortcuts, newShortcuts })
        .then(() => {
          appSettingsDispatch({
            type: AppSettingsActionType.SET_SHORTCUTS,
            payload: { shortcuts: newShortcuts },
          });
        })
        .catch((error) => {
          console.error(error);
          toast.error("Failed to update shortcuts");
        }),
    [appSettingsDispatch],
  );
}
