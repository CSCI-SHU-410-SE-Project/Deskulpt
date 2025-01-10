import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { Appearance } from "../../types/backend";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { Dispatch, SetStateAction } from "react";
import { emitSwitchAppearanceToCanvas } from "../../events";

export interface AppearanceTogglerProps {
  /** Theme appearance. */
  appearance: Appearance;
  /** State for theme appearance. */
  setAppearance: Dispatch<SetStateAction<Appearance>>;
}

/**
 * The theme appearance toggler component.
 *
 * This component will be a small icon bottom on the top right corner of the manager
 * window. Clicking the icon button should switch the theme appearance between light
 * and dark mode. The tooltip and icon should reflect the current theme appearance.
 */
export default function AppearanceToggler({
  appearance,
  setAppearance,
}: AppearanceTogglerProps) {
  const toggleAppearance = () => {
    const newAppearance = appearance === "light" ? "dark" : "light";
    setAppearance(newAppearance);
    emitSwitchAppearanceToCanvas(newAppearance).catch(console.error);
  };

  return (
    <Box position="absolute" right="3" top="4">
      <Tooltip
        side="left"
        content={`Switch to ${appearance === "light" ? "dark" : "light"} mode`}
      >
        <IconButton variant="soft" size="1" onClick={toggleAppearance}>
          {appearance === "light" ? <MdOutlineLightMode /> : <MdOutlineDarkMode />}
        </IconButton>
      </Tooltip>
    </Box>
  );
}
