import { useEffect } from "react";
import { rescan } from "./useWidgetsStore";

export function useInitialRescan() {
  useEffect(() => {
    rescan(true).catch(console.error);
  }, []);
}
