import { useEffect } from "react";
import { listenToRemoveWidgets } from "../../events";
import { removeWidgets } from "./useWidgetsStore";

export function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = listenToRemoveWidgets((event) => {
      const { ids } = event.payload;
      removeWidgets(ids);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
