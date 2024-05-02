import { styled } from "@stitches/react";

const panelWidth = "510px";

export const PanelDivider = styled("hr", {
  width: panelWidth,
  margin: "10px 0",
});

export const UpperPanel = styled("div", {
  height: "160px",
  width: panelWidth,
  overflowX: "hidden",
});

export const LowerPanel = styled("div", {
  height: "240px",
  width: panelWidth,
  overflowX: "hidden",
});

export const PanelSectionHeading = styled("div", {
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  marginBottom: "10px",
});

export const ConfigErrorInfo = styled("div", {
  height: "120px",
  paddingRight: "5px",
  fontFamily: "monospace",
  whiteSpace: "pre-wrap",
  overflowY: "auto",
  color: "red",
});

export const TableInfoCell = styled("td", {
  padding: "0 20px 0 0",
  color: "gray",
});

export const TooltipWrapper = styled("div", {
  maxHeight: "120px",
  maxWidth: "300px",
  overflow: "auto",
  scrollbarWidth: "none",
});
