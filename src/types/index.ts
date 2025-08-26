// config.rs
export * from "../bindings/WidgetConfig";

// settings.rs
export * from "../bindings/AppSettings";
export * from "../bindings/AppSettingsUpdate";
export * from "../bindings/CanvasImode";
export * from "../bindings/Settings";
export * from "../bindings/SettingsUpdate";
export * from "../bindings/Shortcuts";
export * from "../bindings/ShortcutsUpdate";
export * from "../bindings/Theme";
export * from "../bindings/WidgetSettings";
export * from "../bindings/WidgetSettingsUpdate";

export type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P];
};
