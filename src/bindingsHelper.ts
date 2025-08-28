import {
  EventCallback,
  emit,
  emitTo,
  listen,
  once,
} from "@tauri-apps/api/event";
import { DeskulptWindow } from "./bindings/types";

export function __makeEventAPI__<T>(name: string) {
  return {
    listen: (cb: EventCallback<T>) => listen(name, cb),
    once: (cb: EventCallback<T>) => once(name, cb),
    emit: (payload: T) => emit(name, payload),
    emitTo: (window: DeskulptWindow, payload: T) =>
      emitTo(window, name, payload),
  };
}
