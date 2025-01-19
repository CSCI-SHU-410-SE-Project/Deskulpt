import { useEffect } from "react";
import { listenToShowToast } from "../../events";
import { toast } from "sonner";

/**
 * Listen and react to the "show-toast" event.
 */
export default function useShowToastListener() {
  useEffect(() => {
    const unlisten = listenToShowToast((event) => {
      if ("success" in event.payload) {
        void toast.success(event.payload.success);
      } else if ("error" in event.payload) {
        void toast.error(event.payload.error);
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
