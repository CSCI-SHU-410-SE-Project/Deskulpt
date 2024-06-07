import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { invokeInitSettings } from "../commands";
import { emitSwitchThemeAppearanceToCanvas } from "../events";
import App from "./App";
import "@radix-ui/themes/styles.css";
import "../custom.css";

invokeInitSettings()
  .then((settings) => {
    emitSwitchThemeAppearanceToCanvas(settings.themeAppearance).catch(console.error);
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App initialSettings={settings} />
      </StrictMode>,
    );
  })
  .catch(console.error);
