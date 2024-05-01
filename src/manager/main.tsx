import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { invoke } from "@tauri-apps/api/core";
import { Settings } from "../types";

invoke<Settings>("init_settings")
  .then((settings) => {
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App initialSettings={settings} />
      </StrictMode>,
    );
  })
  .catch(console.error);
