import React from "@deskulpt-test/react";
import apis from "@deskulpt-test/apis";

export default {
  render() {
    return <div>{apis.fs.isFile("./index.jsx")}</div>;
  },
  width: 100,
  height: 100,
};
