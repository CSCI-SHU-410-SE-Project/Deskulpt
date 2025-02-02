import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { Theme } from "../../types/backend";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { Dispatch, SetStateAction } from "react";
import { emitSwitchThemeToCanvas } from "../../events";

interface AppearanceTogglerProps {
  /** Theme. */
  theme: Theme;
  /** State for theme. */
  setTheme: Dispatch<SetStateAction<Theme>>;
}

/**
 * The theme appearance toggler component.
 *
 * This component will be a small icon bottom on the top right corner of the manager
 * window. Clicking the icon button should switch the theme appearance between light
 * and dark mode. The tooltip and icon should reflect the current theme appearance.
 */
const AppearanceToggler = ({ theme, setTheme }: AppearanceTogglerProps) => {
  const toggleTheme = () => {
    const newTheme = theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT;
    setTheme(newTheme);
    emitSwitchThemeToCanvas(newTheme).catch(console.error);
  };

  return (
    <Box position="absolute" right="3" top="4">
      <Tooltip
        side="left"
        content={`Switch to ${theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT} mode`}
      >
        <IconButton variant="soft" size="1" onClick={toggleTheme}>
          {theme === Theme.LIGHT ? (
            <MdOutlineLightMode />
          ) : (
            <MdOutlineDarkMode />
          )}
        </IconButton>
      </Tooltip>
    </Box>
  );
};

export default AppearanceToggler;
