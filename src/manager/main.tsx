import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { invokeLoadSettings } from "../core/commands";
import { emitSwitchThemeToCanvas } from "../core/events";
import App from "./App";
import "@radix-ui/themes/styles.css";
import "../custom.css";

async function main() {
  const settings = await invokeLoadSettings();
  await emitSwitchThemeToCanvas({ theme: settings.theme });

  createRoot(document.querySelector("#root")!).render(
    <StrictMode>
      <App settings={settings} />
    </StrictMode>,
  );
}

main().catch(console.error);
