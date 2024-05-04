import { useEffect } from "react";
import { listenToShowToast } from "../events";
import { message } from "antd";

export function useShowToastListener() {
  const [messageApi, contextHolder] = message.useMessage({ maxCount: 1 });

  useEffect(() => {
    const unlisten = listenToShowToast((event) => {
      void messageApi.info({ content: event.payload });
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);

  return contextHolder;
}
