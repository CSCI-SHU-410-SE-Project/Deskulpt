import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DeepReadonly } from "../types/frontend";
import { Settings } from "../types/backend";
import App from "./App";
import "@radix-ui/themes/styles.css";

declare global {
  interface Window {
    readonly __DESKULPT_CANVAS_INTERNALS__: {
      readonly apisWrapper: string;
      readonly initialSettings: DeepReadonly<Settings>;
    };
  }
}

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
