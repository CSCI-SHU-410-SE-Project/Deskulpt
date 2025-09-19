import { useEffect } from "react";
import { events } from "../../bindings";
import { useSettings } from "./useStores";
import { toast } from "sonner";

export function useEventListeners() {
  useUpdateSettingsListener();
  useRenderWidgetsListener();
  useShowToastListener();
}

function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = events.updateSettingsEvent.listen((event) => {
      useSettings.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}

function useRenderWidgetsListener() {
  useEffect(() => {
    const unlisten = events.renderWidgetsEvent.listen(async (event) => {
      event.payload;
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}

function useShowToastListener() {
  useEffect(() => {
    const unlisten = events.showToastEvent.listen((event) => {
      const { type, content } = event.payload;
      switch (type) {
        case "success":
          void toast.success(content);
          break;
        case "error":
          void toast.error(content);
          break;
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
