import { useEffect } from "react";
import { listenToRemoveWidgets } from "../../events";
import { removeWidgets } from "./useWidgetsStore";

export function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = listenToRemoveWidgets((event) => {
      removeWidgets(event.payload.ids);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
