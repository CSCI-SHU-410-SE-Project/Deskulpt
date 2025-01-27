import { ActionDispatch, FC, createElement, useReducer } from "react";
import { WidgetSettings } from "../../types";
import { ErrorDisplay } from "../components";
import { stringifyError } from "../utils";

export interface Widget {
  Component: FC<{ id: string }>;
  width?: string;
  height?: string;
}

export interface WidgetState extends Widget, WidgetSettings {
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export type WidgetsState = Record<string, WidgetState>;

export enum WidgetsActionType {
  SET_RENDER = "SET_RENDER",
  SET_RENDER_ERROR = "SET_RENDER_ERROR",
  SET_SETTINGS = "SET_SETTINGS",
  BATCH_REMOVE = "BATCH_REMOVE",
}

type WidgetsAction =
  | {
      type: WidgetsActionType.SET_RENDER;
      payload: {
        id: string;
        widget: Widget;
        settings?: WidgetSettings;
        apisBlobUrl: string;
        moduleBlobUrl: string;
      };
    }
  | {
      type: WidgetsActionType.SET_RENDER_ERROR;
      payload: {
        id: string;
        error: unknown;
        settings?: WidgetSettings;
        apisBlobUrl: string;
      };
    }
  | {
      type: WidgetsActionType.SET_SETTINGS;
      payload: { id: string; settings: Partial<WidgetSettings> };
    }
  | { type: WidgetsActionType.BATCH_REMOVE; payload: { ids: string[] } };

export type WidgetsDispatch = ActionDispatch<[action: WidgetsAction]>;

export function useWidgets() {
  return useReducer((state: WidgetsState, action: WidgetsAction) => {
    switch (action.type) {
      case WidgetsActionType.SET_RENDER:
        if (action.payload.id in state) {
          return {
            ...state,
            [action.payload.id]: {
              ...state[action.payload.id],
              // Not using spread syntax because we want undefined properties in
              // the widget to override previous properties as well
              Component: action.payload.widget.Component,
              width: action.payload.widget.width,
              height: action.payload.widget.height,
              moduleBlobUrl: action.payload.moduleBlobUrl,
            },
          };
        }
        if (action.payload.settings !== undefined) {
          return {
            ...state,
            [action.payload.id]: {
              ...action.payload.widget,
              ...action.payload.settings,
              apisBlobUrl: action.payload.apisBlobUrl,
              moduleBlobUrl: action.payload.moduleBlobUrl,
            },
          };
        }
        return state;
      case WidgetsActionType.SET_RENDER_ERROR:
        if (action.payload.id in state) {
          return {
            ...state,
            [action.payload.id]: {
              ...state[action.payload.id],
              Component: () =>
                createElement(ErrorDisplay, {
                  id: action.payload.id,
                  error: stringifyError(action.payload.error),
                }),
              width: undefined,
              height: undefined,
              moduleBlobUrl: undefined,
            },
          };
        }
        if (action.payload.settings !== undefined) {
          return {
            ...state,
            [action.payload.id]: {
              ...action.payload.settings,
              Component: () =>
                createElement(ErrorDisplay, {
                  id: action.payload.id,
                  error: stringifyError(action.payload.error),
                }),
              apisBlobUrl: action.payload.apisBlobUrl,
            },
          };
        }
        return state;
      case WidgetsActionType.SET_SETTINGS:
        if (action.payload.id in state) {
          return {
            ...state,
            [action.payload.id]: {
              ...state[action.payload.id],
              ...action.payload.settings,
            },
          };
        }
        return state;
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
