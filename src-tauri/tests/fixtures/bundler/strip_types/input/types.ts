import React from "@deskulpt-test/react";

export type RenderFunction = () => React.ReactNode;

export interface Component {
  render: RenderFunction;
  width: string;
  height: string;
}
