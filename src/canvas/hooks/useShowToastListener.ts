import { useEffect, useRef } from "react";
import { listenToShowToast } from "../../core/events";
import { toast } from "sonner";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";

export function useShowToastListener(ready: ReadyCallback) {
  const isReady = useRef(false);

  useEffect(() => {
    const unlisten = listenToShowToast((event) => {
      if ("success" in event.payload) {
        void toast.success(event.payload.success);
      } else if ("error" in event.payload) {
        void toast.error(event.payload.error);
      }
    });

    if (!isReady.current) {
      isReady.current = true;
      ready(ListenerKeys.SHOW_TOAST);
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [ready]);
}
