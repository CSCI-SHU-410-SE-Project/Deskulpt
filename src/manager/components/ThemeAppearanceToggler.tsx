import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { ThemeAppearance } from "../../types/backend";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { Dispatch, SetStateAction } from "react";
import { emitSwitchThemeAppearanceToCanvas } from "../../events";

export interface ThemeAppearanceTogglerProps {
  /** Theme appearance. */
  themeAppearance: ThemeAppearance;
  /** State for theme appearance. */
  setThemeAppearance: Dispatch<SetStateAction<ThemeAppearance>>;
}

/**
 * The theme appearance toggler component.
 *
 * This component will be a small icon bottom on the top right corner of the manager
 * window. Clicking the icon button should switch the theme appearance between light
 * and dark mode. The tooltip and icon should reflect the current theme appearance.
 */
export default function ThemeAppearanceToggler({
  themeAppearance,
  setThemeAppearance,
}: ThemeAppearanceTogglerProps) {
  const toggleThemeAppearance = () => {
    const newAppearance = themeAppearance === "light" ? "dark" : "light";
    setThemeAppearance(newAppearance);
    emitSwitchThemeAppearanceToCanvas(newAppearance).catch(console.error);
  };

  return (
    <Box position="absolute" right="3" top="2">
      <Tooltip
        side="left"
        content={`Switch to ${themeAppearance === "light" ? "dark" : "light"} mode`}
      >
        <IconButton variant="soft" size="1" onClick={toggleThemeAppearance}>
          {themeAppearance === "light" ? <MdOutlineLightMode /> : <MdOutlineDarkMode />}
        </IconButton>
      </Tooltip>
    </Box>
  );
}
