import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { Theme } from "../../types/backend";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { Dispatch, SetStateAction } from "react";
import { emitSwitchThemeToCanvas } from "../../core/events";

interface Props {
  theme: Theme;
  setTheme: Dispatch<SetStateAction<Theme>>;
}

export default ({ theme, setTheme }: Props) => {
  const toggleTheme = () => {
    const newTheme = theme === "light" ? "dark" : "light";
    setTheme(newTheme);
    emitSwitchThemeToCanvas({
      theme: newTheme,
    }).catch(console.error);
  };

  return (
    <Box position="absolute" right="3" top="4">
      <Tooltip
        side="left"
        content={`Switch to ${theme === "light" ? "dark" : "light"} mode`}
      >
        <IconButton variant="soft" size="1" onClick={toggleTheme}>
          {theme === "light" ? <MdOutlineLightMode /> : <MdOutlineDarkMode />}
        </IconButton>
      </Tooltip>
    </Box>
  );
};
