import { styled } from "@stitches/react";
import { GripVertical } from "lucide-react";

export const Wrapper = styled("div", {
  overflow: "hidden",
  borderRadius: "5px",
  padding: "5px 10px",
  backgroundColor: "rgba(0, 0, 0, 0.7)",
  color: "#cccccc",
  boxShadow: "0 0 2px #888888",
});

export const DragHandle = styled(GripVertical, {
  position: "absolute",
  top: "5px",
  right: "5px",
  width: "20px",
  height: "20px",
  cursor: "grab",
  opacity: "0",
  transition: "opacity 0.3s ease-in-out",
  "&:hover": {
    opacity: "1",
  },
});
