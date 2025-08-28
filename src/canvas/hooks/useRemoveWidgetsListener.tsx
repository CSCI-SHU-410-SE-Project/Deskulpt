import { useEffect } from "react";
import { removeWidgets } from "./useWidgetsStore";
import { RemoveWidgetsEventAPI } from "../../bindings/events";

export function useRemoveWidgetsListener() {
  useEffect(() => {
    const unlisten = RemoveWidgetsEventAPI.listen((event) => {
      removeWidgets(event.payload);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
