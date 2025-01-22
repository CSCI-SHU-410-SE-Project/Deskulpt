import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "@radix-ui/themes/styles.css";

declare global {
  interface Window {
    __DESKULPT_INTERNALS__: {
      apisTemplate: string;
    };
  }
}

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
