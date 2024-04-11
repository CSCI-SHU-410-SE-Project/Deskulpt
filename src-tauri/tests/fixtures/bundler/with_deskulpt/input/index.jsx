const React = window.__DESKULPT__.defaultDeps.React;

import { readFile } from "@deskulpt/api/fs";

function Counter() {
  const [count, setCount] = React.useState(0);

  function handleClick() {
    readFile("myfile.txt");
    setCount(count + 1);
  }

  return (
    <button onClick={handleClick}>
      You pressed me {count} times!
    </button>
  );
}

const App = {
  render: () => <Counter />,
};

export default App;
