import { EventCallback, listen } from "@tauri-apps/api/event";

type ShowToastPayload = { success: string } | { error: string };

export function listenToShowToast(handler: EventCallback<ShowToastPayload>) {
  return listen("show-toast", handler);
}
