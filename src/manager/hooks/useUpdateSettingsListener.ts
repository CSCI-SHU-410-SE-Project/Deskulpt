import { Dispatch, SetStateAction, useEffect } from "react";
import { listenToUpdateSettings } from "../../events";
import { ManagerWidgetState } from "../../types/frontend";

/**
 * Listen and react to the "update-setting" event.
 *
 * This hook listens to the "update-setting" event and updates the per-widget setting
 * part of the manager widget states. If the given widget ID is not in the collection,
 * nothing will be updated.
 *
 * @param setManagerWidgetStates Setter for the manager widget states.
 */
export default function useUpdateSettingListener(
  setManagerWidgetStates: Dispatch<
    SetStateAction<Record<string, ManagerWidgetState>>
  >,
) {
  useEffect(() => {
    const unlisten = listenToUpdateSettings((event) => {
      const { id, settings } = event.payload;

      setManagerWidgetStates((prev) => {
        if (id in prev) {
          return { ...prev, [id]: { ...prev[id], settings } };
        }
        return prev;
      });
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [setManagerWidgetStates]);
}
