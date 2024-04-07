// This file does not have React defined, but its imported file does

const App = {
  render: () => <Message />,
};

import MessageInner from "./MessageInner";

function Message() {
  return <div><MessageInner /></div>;
}

export default App;
