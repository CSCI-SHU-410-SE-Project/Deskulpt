import { getMessage } from "./utils.js";
import React from "@deskulpt-test/react";

function Message() {
  return <div>{getMessage()}</div>;
}

const App = {
  render: () => <Message />,
};

export default App;
