import { RenderFunction, Component } from "./types";

const render: RenderFunction = () => <div>Hello, world!</div>;

export default {
  render,
  width: "100px",
  height: "100px",
} satisfies Component;
