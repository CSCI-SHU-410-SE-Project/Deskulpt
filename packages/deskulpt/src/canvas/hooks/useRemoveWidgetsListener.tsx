import { useEffect } from "react";
import { removeWidgets } from "./useWidgetsStore";
import { events } from "../../bindings";

export function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = events.removeWidgetsEvent.listen((event) => {
      removeWidgets(event.payload);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
