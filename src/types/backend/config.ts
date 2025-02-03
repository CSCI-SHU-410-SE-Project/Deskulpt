export enum WidgetConfigType {
  VALID = "VALID",
  INVALID = "INVALID",
}

export type WidgetConfig =
  | {
      type: WidgetConfigType.VALID;
      content: {
        dir: string;
        name: string;
        entry: string;
        dependencies: Record<string, string>;
      };
    }
  | {
      type: WidgetConfigType.INVALID;
      content: {
        dir: string;
        error: string;
      };
    };
