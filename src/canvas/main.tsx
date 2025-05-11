import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DeepReadonly, Settings } from "../types";
import App from "./App";
import "@radix-ui/themes/styles.css";
import { enforceOpenNewTab } from "../util/useEnforceOpenNewTab";

declare global {
  interface Window {
    readonly __DESKULPT_CANVAS_INTERNALS__: {
      readonly apisWrapper: string;
      readonly initialSettings: DeepReadonly<Settings>;
    };
  }
}

enforceOpenNewTab();

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
