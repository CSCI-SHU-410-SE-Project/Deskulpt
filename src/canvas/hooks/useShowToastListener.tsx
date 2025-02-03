import { useEffect } from "react";
import { events } from "../../core";
import { toast } from "sonner";

export function useShowToastListener() {
  useEffect(() => {
    const unlisten = events.showToast.on((event) => {
      const { type, content } = event.payload;
      switch (type) {
        case events.ShowToastPayloadType.SUCCESS:
          void toast.success(content);
          break;
        case events.ShowToastPayloadType.ERROR:
          void toast.error(content);
          break;
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
