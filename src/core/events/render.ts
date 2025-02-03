import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import { WidgetSettings } from "../../types";

export type RenderPayload = {
  id: string;
  settings?: WidgetSettings;
  code?: string;
}[];

export async function emitRenderToCanvas(payload: RenderPayload) {
  await emitTo("canvas", "render", payload);
}

export function listenToRender(handler: EventCallback<RenderPayload>) {
  return listen("render", handler);
}
