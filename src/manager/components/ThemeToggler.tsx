import { Box, IconButton } from "@radix-ui/themes";
import { Theme } from "../../bindings/types";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { toggleTheme } from "../hooks";

interface ThemeTogglerProps {
  theme: Theme;
}

const ThemeToggler = ({ theme }: ThemeTogglerProps) => {
  return (
    <Box position="absolute" right="3" top="4">
      <IconButton
        title="Toggle theme"
        variant="soft"
        size="1"
        onClick={toggleTheme}
      >
        {theme === "light" ? <MdOutlineLightMode /> : <MdOutlineDarkMode />}
      </IconButton>
    </Box>
  );
};

export default ThemeToggler;
