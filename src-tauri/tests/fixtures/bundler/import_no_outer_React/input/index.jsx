import getMessage from "./utils";

function Message() {
  return (
    <div>{getMessage()}</div>
  );
}

const App = {
  render: () => <Message />,
};

export default App;
