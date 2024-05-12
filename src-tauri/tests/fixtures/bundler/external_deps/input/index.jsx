import React from "@deskulpt-test/react";
import osName from "os-name";
import { matcher } from "matcher";

export default {
  render() {
    return (
      <div>
        <p>Your OS: {osName()}</p>
        <p>Matcher: {matcher(["foo", "bar", "baz"], "b*")}</p>
      </div>
    );
  },
  width: 100,
  height: 100,
};
