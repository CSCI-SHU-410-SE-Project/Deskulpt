import { useCallback, useEffect, useRef, useState } from "react";
import { invokeWindowReady } from "../../core/commands";

export enum ListenerKeys {
  EXIT_APP = "exit-app",
  UPDATE_SETTINGS = "update-settings",
  WINDOW_READY = "window-ready",
}

export type ReadyCallback = (key: ListenerKeys) => void;

export function useListenersReady() {
  const allReady = useRef(false);

  const [listenersReady, setListenersReady] = useState({
    [ListenerKeys.EXIT_APP]: false,
    [ListenerKeys.UPDATE_SETTINGS]: false,
    [ListenerKeys.WINDOW_READY]: false,
  });

  const ready = useCallback((key: ListenerKeys) => {
    setListenersReady((prev) => ({ ...prev, [key]: true }));
  }, []);

  useEffect(() => {
    if (
      listenersReady[ListenerKeys.EXIT_APP] &&
      listenersReady[ListenerKeys.UPDATE_SETTINGS] &&
      listenersReady[ListenerKeys.WINDOW_READY] &&
      !allReady.current
    ) {
      invokeWindowReady({ window: "manager" })
        .then(() => {
          allReady.current = true;
        })
        .catch(console.error);
    }
  }, [listenersReady]);

  return ready;
}
