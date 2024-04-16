const App = {
  render: () => <Message />,
};

import MessageInner from "./MessageInner";

function Message() {
  return <div><MessageInner /></div>;
}

export default App;
