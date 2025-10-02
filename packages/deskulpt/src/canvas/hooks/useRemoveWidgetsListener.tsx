import { useEffect } from "react";
import { removeWidgets } from "./useWidgetsStore";
import { events } from "../../bindings";

export function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = events.removeWidgets.listen((event) => {
      removeWidgets(event.payload);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
