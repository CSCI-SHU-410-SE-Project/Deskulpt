import React from "react";
import ReactDOM from "react-dom/client";
import { WidgetInternal } from "../types";
import App from "./App";
import { invoke } from "@tauri-apps/api";

invoke<Record<string, WidgetInternal>>("get_widget_internals")
  .then((widgetInternals) => {
    ReactDOM.createRoot(document.getElementById("canvas")!).render(
      <React.StrictMode>
        <App initialInternals={widgetInternals} />
      </React.StrictMode>,
    );
  })
  .catch(console.error);
