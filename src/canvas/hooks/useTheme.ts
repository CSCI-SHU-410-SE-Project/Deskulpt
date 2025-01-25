import { useEffect, useRef, useState } from "react";
import { listenToSwitchTheme } from "../../core/events";
import { ListenerKeys, ReadyCallback } from "./useListenersReady";
import { Theme } from "../../types";

export function useTheme(ready: ReadyCallback) {
  const isReady = useRef(false);

  const [theme, setTheme] = useState(
    window.__DESKULPT__.initialSettings.app.theme,
  );

  useEffect(() => {
    const unlisten = listenToSwitchTheme(() => {
      setTheme((prevTheme) =>
        prevTheme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT,
      );
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
