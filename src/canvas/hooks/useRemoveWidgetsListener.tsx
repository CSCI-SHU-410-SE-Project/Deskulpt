import { useEffect } from "react";
import { events } from "../../core";
import { removeWidgets } from "./useWidgetsStore";

export function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = events.removeWidgets.on((event) => {
      removeWidgets(event.payload.ids);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
