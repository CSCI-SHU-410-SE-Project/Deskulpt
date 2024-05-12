import React from "@deskulpt-test/react";

type RenderFunction = () => React.ReactNode;

interface Component {
  render: RenderFunction;
  width: number;
  height: number;
}

export default {
  render: () => (
    <div>Hello, world!</div>
  ),
  width: 100,
  height: 100,
} satisfies Component;
