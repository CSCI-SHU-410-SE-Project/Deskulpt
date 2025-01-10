import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { ReactNode } from "react";

export interface FloatButtonProps {
  /** The order of the button, i.e., `(order - 1) * 40` pixels away from the bottom. */
  order: number;
  /** The icon to display in the float button. */
  icon: ReactNode;
  /** The tooltip text to show when hovering the float button. */
  tooltip: string;
  /** The click action of the float button. */
  onClick: () => void;
  /** Whether the float button is disabled. */
  disabled?: boolean;
}

/**
 * The float button component in the bottom right corner.
 *
 * This will be a circular icon button with a tooltip on hover, rendered in the bottom
 * right corner of the window.
 */
export default function FloatButton({
  order,
  icon,
  tooltip,
  onClick,
  disabled,
}: FloatButtonProps) {
  return (
    <Box position="absolute" right="0" bottom={`${(order - 1) * 40}px`}>
      <Tooltip content={tooltip} side="left">
        <IconButton
          variant="surface"
          radius="full"
          disabled={disabled}
          onClick={onClick}
        >
          {icon}
        </IconButton>
      </Tooltip>
    </Box>
  );
}
