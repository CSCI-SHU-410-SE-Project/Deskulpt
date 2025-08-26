// config.rs
export * from "../bindings/WidgetConfig";

// settings.rs
export * from "../bindings/AppSettings";
export * from "../bindings/Settings";
export * from "../bindings/Shortcuts";
export * from "../bindings/Theme";
export * from "../bindings/WidgetSettings";

export type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P];
};
