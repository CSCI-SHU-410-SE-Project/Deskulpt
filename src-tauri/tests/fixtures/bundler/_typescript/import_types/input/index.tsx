import React from "@deskulpt-test/react";
import { RenderFunction, Component } from "./types";

const render: RenderFunction = () => (
  <div>Hello, world!</div>
);

export default {
  render,
  width: 100,
  height: 100,
} satisfies Component;
