import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { invokeLoadSettings } from "../commands";
import { emitSwitchAppearanceToCanvas } from "../events";
import App from "./App";
import "@radix-ui/themes/styles.css";
import "../custom.css";

invokeLoadSettings()
  .then((settings) => {
    emitSwitchAppearanceToCanvas(settings.appearance).catch(console.error);
    createRoot(document.querySelector("#root")!).render(
      <StrictMode>
        <App initialSettings={settings} />
      </StrictMode>,
    );
  })
  // This command never fails by design so we do not need to handle the error case
  .catch(console.error);
