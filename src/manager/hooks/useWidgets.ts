import { ActionDispatch, useReducer } from "react";
import { WidgetConfig, WidgetSettings } from "../../types";

export interface WidgetState {
  id: string;
  config: WidgetConfig;
  settings: WidgetSettings;
}

export type WidgetsState = WidgetState[];

export enum WidgetsActionType {
  RESET_ALL = "RESET_ALL",
  SET_SETTINGS = "SET_SETTINGS",
  BATCH_REMOVE = "BATCH_REMOVE",
}

type WidgetsAction =
  | {
      type: WidgetsActionType.RESET_ALL;
      payload: { widgets: WidgetsState };
    }
  | {
      type: WidgetsActionType.SET_SETTINGS;
      payload: { id: string; settings: Partial<WidgetSettings> };
    }
  | {
      type: WidgetsActionType.BATCH_REMOVE;
      payload: { ids: string[] };
    };

export type WidgetsDispatch = ActionDispatch<[action: WidgetsAction]>;

export function useWidgets() {
  return useReducer((state: WidgetsState, action: WidgetsAction) => {
    switch (action.type) {
      case WidgetsActionType.RESET_ALL:
        return action.payload.widgets.sort((a, b) =>
          a.config.content.dir.localeCompare(b.config.content.dir),
        );
      case WidgetsActionType.SET_SETTINGS: {
        const index = state.findIndex(({ id }) => id === action.payload.id);
        if (index === -1) return state;
        const newState = [...state];
        newState[index] = {
          ...state[index],
          settings: {
            ...state[index].settings,
            ...action.payload.settings,
          },
        };
        return newState;
      }
      case WidgetsActionType.BATCH_REMOVE:
        return state.filter(({ id }) => !action.payload.ids.includes(id));
      default:
        throw new Error("Invalid action type");
    }
  }, []);
}
