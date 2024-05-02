import { styled } from "@stitches/react";

export const ScrollArea = styled("div", {
  overflow: "auto",
  scrollbarWidth: "none",
  width: "100%",
  height: "100%",
});

export const ErrorTitle = styled("div", {
  fontWeight: "bold",
  color: "red",
});

export const ErrorBody = styled("div", {
  whiteSpace: "pre-wrap",
  fontFamily: "monospace",
});
