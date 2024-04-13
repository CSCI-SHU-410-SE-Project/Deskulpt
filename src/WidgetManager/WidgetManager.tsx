// import React from 'react';
import Button from "@mui/material/Button";
import { invoke } from "@tauri-apps/api/tauri";

import "./WidgetManager.css";

(function () {
  // add a script element that log a message to the console
  const script = document.createElement("script");
  script.type = "module";
  script.text = `
window.__TAURI_IPC__({ cmd: 'test', callback: 0, error: 0});
	`;
  document.body.appendChild(script);
})();

const WidgetManager = () => {
  return (
    <div style={{ border: "1px solid black", padding: "10px" }}>
      <h1>Widget Manager</h1>
      <div>
        <Button
          variant="contained"
          color="primary"
          onClick={() => invoke("sink_canvas")}
        >
          Sink Canvas
        </Button>
        <Button
          variant="contained"
          color="primary"
          onClick={() => invoke("float_canvas")}
        >
          Float Canvas
        </Button>
      </div>
    </div>
  );
};

export default WidgetManager;
