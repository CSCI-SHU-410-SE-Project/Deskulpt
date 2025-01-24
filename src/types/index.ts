import { Settings } from "./backend";

export * from "./backend";

type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P];
};

declare global {
  interface Window {
    readonly __DESKULPT__: {
      readonly apisWrapper: string;
      readonly initialSettings: DeepReadonly<Settings>;
    };
  }
}
