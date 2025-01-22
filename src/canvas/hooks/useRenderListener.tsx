import { useEffect } from "react";
import { listenToRenderWidget } from "../../core/events";
import { RenderCallback } from "./useRenderCallback";

export function useRenderListener(render: RenderCallback) {
  useEffect(() => {
    const unlisten = listenToRenderWidget((event) => {
      const { id, settings, code } = event.payload;
      render(id, settings, code);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, [render]);
}
