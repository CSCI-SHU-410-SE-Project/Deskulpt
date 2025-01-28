import { ActionDispatch, useReducer } from "react";
import { WidgetConfig, WidgetSettings } from "../../types";

export interface WidgetState {
  id: string;
  config: WidgetConfig;
  settings: WidgetSettings;
}

export type WidgetsState = WidgetState[];

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
      case WidgetsActionType.BATCH_UPDATE: {
        // Use an object to deduplicate widgets by ID and resort the whole array
        // by widget directory name; array size will not be too large to cause
        // performance issues
        const map = {} as Record<string, WidgetState>;
        state.forEach((widget) => {
          map[widget.id] = widget;
        });
        action.payload.widgets.forEach((widget) => {
          map[widget.id] = widget;
        });
        return Object.values(map).sort((a, b) =>
          a.config.content.dir.localeCompare(b.config.content.dir),
        );
      }
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
