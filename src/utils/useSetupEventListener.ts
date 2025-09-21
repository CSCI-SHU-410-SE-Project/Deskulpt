import { UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useRef } from "react";
import { SetupTask, commands } from "../bindings";

export function useSetupEventListener(
  task: SetupTask,
  listener: () => Promise<UnlistenFn>,
) {
  const hasInited = useRef(false);

  useEffect(() => {
    const unlisten = listener();

    if (!hasInited.current) {
      commands
        .markSetup({ task })
        .then(() => {
          hasInited.current = true;
        })
        .catch(console.error);
    }

    return () => {
      hasInited.current = false;
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [task, listener]);
}
