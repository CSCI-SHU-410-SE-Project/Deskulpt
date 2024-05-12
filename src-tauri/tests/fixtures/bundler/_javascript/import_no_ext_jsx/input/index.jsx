import MessageInner from "./MessageInner";
import React from "@deskulpt-test/react";

function Message() {
  return <div><MessageInner /></div>;
}

const App = {
  render: () => <Message />,
  width: 100,
  height: 100,
};

export default App;
