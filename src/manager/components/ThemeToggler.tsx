import { Box, IconButton } from "@radix-ui/themes";
import { Theme } from "../../types";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { toggleTheme } from "../hooks";

interface AppearanceTogglerProps {
  theme: Theme;
}

const AppearanceToggler = ({ theme }: AppearanceTogglerProps) => {
  return (
    <Box position="absolute" right="3" top="4">
      <IconButton
        title="Toggle theme"
        variant="soft"
        size="1"
        onClick={toggleTheme}
      >
        {theme === Theme.LIGHT ? <MdOutlineLightMode /> : <MdOutlineDarkMode />}
      </IconButton>
    </Box>
  );
};

export default AppearanceToggler;
