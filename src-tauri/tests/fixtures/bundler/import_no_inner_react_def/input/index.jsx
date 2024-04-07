// This file has React defined, but its imported file does not

import MessageInner from "./MessageInner";

const React = window.__DESKULPT__.defaultDeps.React;

function Message() {
  return <div><MessageInner /></div>
}

const App = {
  render: () => <Message />,
};

export default App;
