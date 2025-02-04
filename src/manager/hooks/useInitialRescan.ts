import { useEffect } from "react";
import { rescan } from "./useWidgetsStore";

export default function useInitialRescan() {
  useEffect(() => {
    rescan(true).catch(console.error);
  }, []);
}
