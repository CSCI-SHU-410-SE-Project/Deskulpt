import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { invokeInitGlobalSetting } from "../commands";
import { emitSwitchThemeAppearanceToCanvas } from "../events";
import App from "./App";
import "@radix-ui/themes/styles.css";
import "../custom.css";

invokeInitGlobalSetting()
  .then((globalSetting) => {
    emitSwitchThemeAppearanceToCanvas(globalSetting.themeAppearance).catch(
      console.error,
    );
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App initialGlobalSetting={globalSetting} />
      </StrictMode>,
    );
  })
  .catch(console.error);
