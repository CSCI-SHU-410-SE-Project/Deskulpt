import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { enforceOpenNewTab } from "../utils/enforceOpenNewTab";
import WidgetApp from "./WidgetApp";
import "@radix-ui/themes/styles.css";

enforceOpenNewTab();

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <WidgetApp />
  </StrictMode>,
);
