import { ActionDispatch, useReducer } from "react";
import { WidgetConfig, WidgetSettings } from "../../types";

export interface WidgetState {
  config: WidgetConfig;
  settings: WidgetSettings;
}

export type WidgetsState = Record<string, WidgetState>;

export enum WidgetsActionType {
  BATCH_UPDATE = "BATCH_UPDATE",
  SET_SETTINGS = "SET_SETTINGS",
  BATCH_REMOVE = "BATCH_REMOVE",
}

type WidgetsAction =
  | {
      type: WidgetsActionType.BATCH_UPDATE;
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
      case WidgetsActionType.BATCH_UPDATE:
        return { ...state, ...action.payload.widgets };
      case WidgetsActionType.SET_SETTINGS:
        if (!(action.payload.id in state)) return state;
        return {
          ...state,
          [action.payload.id]: {
            ...state[action.payload.id],
            settings: {
              ...state[action.payload.id].settings,
              ...action.payload.settings,
            },
          },
        };
      case WidgetsActionType.BATCH_REMOVE:
        return Object.fromEntries(
          Object.entries(state).filter(
            ([id]) => !action.payload.ids.includes(id),
          ),
        );
      default:
        throw new Error("Invalid action type");
    }
  }, {});
}
