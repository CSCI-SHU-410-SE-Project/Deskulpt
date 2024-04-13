import WidgetCanvas from "../WidgetCanvas/WidgetCanvas";

// import { emit } from '@tauri-apps/api/event';

import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";

// document.getElementById("root")?.addEventListener("click", () => {
//   emit("click", { message: "You have clicked the root" });
// });

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <WidgetCanvas />
  </React.StrictMode>,
);
