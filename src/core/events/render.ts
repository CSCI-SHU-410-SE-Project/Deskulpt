import { EventCallback, emitTo, listen } from "@tauri-apps/api/event";
import { WidgetSettings } from "../../types";

interface RenderWidgetPayload {
  id: string;
  settings?: WidgetSettings;
  code?: string;
}

export async function emitRenderToCanvas(payload: RenderWidgetPayload) {
  await emitTo("canvas", "render", payload);
}

export function listenToRender(handler: EventCallback<RenderWidgetPayload>) {
  return listen("render", handler);
}
