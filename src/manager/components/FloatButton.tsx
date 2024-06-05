import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { ReactNode } from "react";

interface FloatButtonProps {
  order: number;
  icon: ReactNode;
  tooltip: string;
  onClick: () => void;
}

export default function FloatButton({
  order,
  icon,
  tooltip,
  onClick,
}: FloatButtonProps) {
  return (
    <Box position="absolute" right="3" bottom={`${(order - 1) * 40}px`}>
      <Tooltip content={tooltip} side="left">
        <IconButton variant="surface" radius="full" onClick={onClick}>
          {icon}
        </IconButton>
      </Tooltip>
    </Box>
  );
}
