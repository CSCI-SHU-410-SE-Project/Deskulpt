import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { Theme } from "../../types";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { ToggleThemeCallback } from "../hooks";

interface Props {
  theme: Theme;
  toggleTheme: ToggleThemeCallback;
}

export default ({ theme, toggleTheme }: Props) => {
  return (
    <Box position="absolute" right="3" top="4">
      <Tooltip side="left" content={`Switch to ${theme} mode`}>
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
