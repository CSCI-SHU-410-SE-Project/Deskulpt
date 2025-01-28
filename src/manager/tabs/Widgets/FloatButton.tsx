import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { ReactNode } from "react";

interface Props {
  bottom: string;
  icon: ReactNode;
  tooltip: string;
  onClick: () => void;
  disabled?: boolean;
}

export default ({ bottom, icon, tooltip, onClick, disabled }: Props) => {
  return (
    <Box position="absolute" right="0" bottom={bottom}>
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
};
