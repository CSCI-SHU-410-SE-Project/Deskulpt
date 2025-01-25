import { Box, IconButton, Tooltip } from "@radix-ui/themes";
import { Theme } from "../../types";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { AppSettingsActionType, AppSettingsDispatch } from "../hooks";
import { useCallback } from "react";
import { emitSwitchThemeToCanvas } from "../../core/events";

interface Props {
  theme: Theme;
  appSettingsDispatch: AppSettingsDispatch;
}

export default ({ theme, appSettingsDispatch }: Props) => {
  const toggleTheme = useCallback(() => {
    appSettingsDispatch({ type: AppSettingsActionType.TOGGLE_THEME });
    emitSwitchThemeToCanvas();
  }, [appSettingsDispatch]);

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
