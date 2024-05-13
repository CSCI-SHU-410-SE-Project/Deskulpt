import React from "@deskulpt-test/react";

function Counter() {
  const [count, setCount] = React.useState(0);

  function handleClick() {
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
  width: 100,
  height: 100,
};

export default App;
