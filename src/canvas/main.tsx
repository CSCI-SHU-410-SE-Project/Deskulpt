import WidgetCanvas from "../WidgetCanvas/WidgetCanvas";

import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <div>
      <WidgetCanvas />
    </div>
  </React.StrictMode>,
);
