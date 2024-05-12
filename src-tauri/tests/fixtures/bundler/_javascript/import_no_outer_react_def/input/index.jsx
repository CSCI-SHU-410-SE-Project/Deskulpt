const App = {
  render: () => <Message />,
  width: 100,
  height: 100,
};

import MessageInner from "./MessageInner";

function Message() {
  return <div><MessageInner /></div>;
}

export default App;
