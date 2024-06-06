import { useEffect } from "react";
import { listenToShowToast } from "../../events";
import { toast } from "sonner";

/**
 * Listen and react to the "show-toast" event.
 */
export default function useShowToastListener() {
  useEffect(() => {
    const unlisten = listenToShowToast((event) => {
      const { kind, message } = event.payload;
      if (kind === "success") {
        void toast.success(message);
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
