import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "@radix-ui/themes/styles.css";

declare global {
  interface Window {
    readonly __DESKULPT_CANVAS_INTERNALS__: {
      readonly apisWrapper: string;
    };
  }
}

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
