import { useEffect } from "react";
import { toast } from "sonner";
import { ShowToastEventAPI } from "../../bindings/events";

export function useShowToastListener() {
  useEffect(() => {
    const unlisten = ShowToastEventAPI.listen((event) => {
      const { type, content } = event.payload;
      switch (type) {
        case "SUCCESS":
          void toast.success(content);
          break;
        case "ERROR":
          void toast.error(content);
          break;
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
