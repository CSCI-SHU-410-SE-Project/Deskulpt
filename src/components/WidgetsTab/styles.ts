import { css } from "@emotion/react";

/** Wrapper of a panel. */
export const panelWrapper = css({
  width: "510px",
  overflowX: "hidden",
});

/** Title of a panel, with two items on two sides.  */
export const panelTitle = css({
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  marginBottom: "10px",
});

/** Caption of table. */
export const tableCaption = css({
  padding: "0 20px 0 0",
  color: "gray",
});
