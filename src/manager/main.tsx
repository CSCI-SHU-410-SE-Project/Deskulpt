import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { invokeInitSettings } from "../commands";
import App from "./App";

invokeInitSettings()
  .then((settings) => {
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App initialSettings={settings} />
      </StrictMode>,
    );
  })
  .catch(console.error);
