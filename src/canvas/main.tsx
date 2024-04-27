import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { WidgetInternal } from "../types";
import App from "./App";
import { invoke } from "@tauri-apps/api";

invoke<Record<string, WidgetInternal>>("init_widget_internals")
  .then((widgetInternals) => {
    createRoot(document.getElementById("canvas")!).render(
      <StrictMode>
        <App initialInternals={widgetInternals} />
      </StrictMode>,
    );
  })
  .catch(console.error);
