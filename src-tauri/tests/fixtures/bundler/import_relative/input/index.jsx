import { getMessage } from "./utils.js";

const React = window.__DESKULPT__.defaultDeps.React;

function Message() {
  return (
    <div>{getMessage()}</div>
  );
}

const App = {
  render: () => <Message />,
};

export default App;
