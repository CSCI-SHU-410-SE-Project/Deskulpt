import { ActionDispatch, useReducer } from "react";
import { AppSettings, Shortcuts, Theme } from "../../types";

export enum AppSettingsActionType {
  TOGGLE_THEME = "TOGGLE_THEME",
  SET_SHORTCUT = "SET_SHORTCUT",
}

type AppSettingsAction =
  | {
      type: AppSettingsActionType.TOGGLE_THEME;
    }
  | {
      type: AppSettingsActionType.SET_SHORTCUT;
      payload: { key: keyof Shortcuts; shortcut: string | null };
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
      case AppSettingsActionType.SET_SHORTCUT:
        return {
          ...state,
          shortcuts: {
            ...state.shortcuts,
            [action.payload.key]: action.payload.shortcut,
          },
        };
      default:
        throw new Error("Invalid action type");
    }
  }, window.__DESKULPT_MANAGER_INTERNALS__.initialSettings.app);
}
