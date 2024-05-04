import { css } from "@emotion/react";

/** Caption of each setting. */
export const caption = css({
  width: "150px",
  color: "gray",
});

/** Item at the end of each setting. */
export const rowEnd = css({
  marginLeft: "auto",
});

/** Keyboard element. */
export const keyboard = css({
  borderRadius: "5px",
  border: "1px solid gray",
  boxShadow: "0 1px 0 1px gray",
  padding: "0 5px",
  marginRight: "5px",
});

/** Flex wrapper that spreads its two children. */
export const spreadWrapper = css({
  display: "flex",
  justifyContent: "space-between",
  alignItems: "center",
  margin: "10px 0",
});

/** Description paragraph of drawer content. */
export const drawerDescription = css({
  fontSize: "0.8rem",
  color: "gray",
  marginBottom: "30px",
});
