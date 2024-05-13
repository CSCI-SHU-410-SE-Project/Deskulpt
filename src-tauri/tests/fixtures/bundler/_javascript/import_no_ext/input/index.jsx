import { getMessage } from "./utils";
import React from "@deskulpt-test/react";

function Message() {
  return <div>{getMessage()}</div>;
}

const App = {
  render: () => <Message />,
  width: 100,
  height: 100,
};

export default App;
