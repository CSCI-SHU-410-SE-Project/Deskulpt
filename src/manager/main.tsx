import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DeepReadonly, Settings } from "../types";
import { enforceOpenNewTab } from "../utils/enforceOpenNewTab";
import App from "./App";
import "@radix-ui/themes/styles.css";
import "../custom.css";

declare global {
  interface Window {
    readonly __DESKULPT_MANAGER_INTERNALS__: {
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
