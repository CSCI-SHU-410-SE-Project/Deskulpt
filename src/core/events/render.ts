import { emitTo, EventCallback, listen } from "@tauri-apps/api/event";
import { WidgetSettings } from "../../types/backend";

interface RenderWidgetPayload {
  id: string;
  settings?: WidgetSettings;
  code?: string;
}

export async function emitRenderToCanvas(payload: RenderWidgetPayload) {
  await emitTo("canvas", "render", payload);
}

export function listenToRenderWidget(
  handler: EventCallback<RenderWidgetPayload>,
) {
  return listen("render", handler);
}
