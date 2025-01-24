import { useEffect, useRef, useState } from "react";
import { listenToSwitchTheme } from "../../core/events";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";

export function useTheme(ready: ReadyCallback) {
  const isReady = useRef(false);
  const [theme, setTheme] = useState(window.__DESKULPT__.initialSettings.theme);

  useEffect(() => {
    const unlisten = listenToSwitchTheme((event) => {
      const { theme } = event.payload;
      setTheme(theme);
    });

    if (!isReady.current) {
      ready(ListenerKeys.SWITCH_THEME);
      isReady.current = true;
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return theme;
}
