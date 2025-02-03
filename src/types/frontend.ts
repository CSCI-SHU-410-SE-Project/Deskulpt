import { WidgetConfig } from "./backend/config";
import { WidgetSettings } from "./backend/settings";

export interface ManagerWidgetState {
  config: WidgetConfig;
  settings: WidgetSettings;
}
