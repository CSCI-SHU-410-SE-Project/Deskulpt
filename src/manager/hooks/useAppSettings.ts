import { ActionDispatch, useReducer } from "react";
import { AppSettings, Shortcuts, Theme } from "../../types";

export enum AppSettingsActionType {
  TOGGLE_THEME = "TOGGLE_THEME",
  SET_SHORTCUTS = "SET_SHORTCUTS",
}

type AppSettingsAction =
  | {
      type: AppSettingsActionType.TOGGLE_THEME;
    }
  | {
      type: AppSettingsActionType.SET_SHORTCUTS;
      payload: { shortcuts: Shortcuts };
    };

export type AppSettingsDispatch = ActionDispatch<[action: AppSettingsAction]>;

export function useAppSettings() {
  return useReducer((state: AppSettings, action: AppSettingsAction) => {
    switch (action.type) {
      case AppSettingsActionType.TOGGLE_THEME:
        return {
          ...state,
          theme: state.theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT,
        };
      case AppSettingsActionType.SET_SHORTCUTS:
        return { ...state, shortcuts: action.payload.shortcuts };
      default:
        throw new Error("Invalid action type");
    }
  }, window.__DESKULPT__.initialSettings.app);
}
