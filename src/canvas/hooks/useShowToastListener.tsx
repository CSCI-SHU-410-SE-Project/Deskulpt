import { useEffect } from "react";
import { listenToShowToast } from "../../events";
import { toast } from "sonner";
import { ShowToastPayloadType } from "../../types/backend";

export function useShowToastListener() {
  useEffect(() => {
    const unlisten = listenToShowToast((event) => {
      const { type, content } = event.payload;
      switch (type) {
        case ShowToastPayloadType.SUCCESS:
          void toast.success(content);
          break;
        case ShowToastPayloadType.ERROR:
          void toast.error(content);
          break;
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
