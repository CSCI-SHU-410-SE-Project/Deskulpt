import { useEffect, useRef } from "react";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";
import { listenToWindowReadyOnce } from "../../core/events";
import { RescanCallback } from "./useRescanCallback";

export function useWindowReadyListener(
  rescan: RescanCallback,
  ready: ReadyCallback,
) {
  const isReady = useRef(false);

  useEffect(() => {
    const unlisten = listenToWindowReadyOnce(rescan);

    if (!isReady.current) {
      ready(ListenerKeys.WINDOW_READY);
      isReady.current = true;
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [rescan]);
}
