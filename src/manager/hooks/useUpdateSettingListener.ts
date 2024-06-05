import { Dispatch, SetStateAction, useEffect } from "react";
import { listenToUpdateSetting } from "../../events";
import { ManagerWidgetState } from "../../types/frontend";
import { IdMap } from "../../types/backend";

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
