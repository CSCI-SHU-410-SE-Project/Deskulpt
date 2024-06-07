import { Dispatch, SetStateAction, useEffect } from "react";
import { listenToUpdateSetting } from "../../events";
import { ManagerWidgetState } from "../../types/frontend";
import { IdMap } from "../../types/backend";

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
  setManagerWidgetStates: Dispatch<SetStateAction<IdMap<ManagerWidgetState>>>,
) {
  useEffect(() => {
    const unlisten = listenToUpdateSetting((event) => {
      const { widgetId, setting } = event.payload;

      setManagerWidgetStates((prev) => {
        if (widgetId in prev) {
          return { ...prev, [widgetId]: { ...prev[widgetId], setting } };
        }
        return prev;
      });
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
