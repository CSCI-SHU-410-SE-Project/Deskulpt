import { useCallback, useEffect, useRef, useState } from "react";
import { invokeWindowReady } from "../../core/commands";

export enum ListenerKeys {
  BATCH_REMOVE = "batch-remove",
  RENDER = "render",
  SHOW_TOAST = "show-toast",
  SWITCH_THEME = "switch-theme",
  UPDATE_SETTINGS = "update-settings",
}

export type ReadyCallback = (key: ListenerKeys) => void;

export function useListenersReady() {
  const allReady = useRef(false);
  const [listenersReady, setListenersReady] = useState({
    [ListenerKeys.BATCH_REMOVE]: false,
    [ListenerKeys.RENDER]: false,
    [ListenerKeys.SHOW_TOAST]: false,
    [ListenerKeys.SWITCH_THEME]: false,
    [ListenerKeys.UPDATE_SETTINGS]: false,
  });

  const ready = useCallback((key: ListenerKeys) => {
    setListenersReady((prev) => ({ ...prev, [key]: true }));
  }, []);

  useEffect(() => {
    if (
      listenersReady[ListenerKeys.BATCH_REMOVE] &&
      listenersReady[ListenerKeys.RENDER] &&
      listenersReady[ListenerKeys.SHOW_TOAST] &&
      listenersReady[ListenerKeys.SWITCH_THEME] &&
      listenersReady[ListenerKeys.UPDATE_SETTINGS] &&
      !allReady.current
    ) {
      invokeWindowReady({ window: "canvas" })
        .then(() => {
          allReady.current = true;
        })
        .catch(console.error);
    }
  }, [listenersReady]);

  return ready;
}
