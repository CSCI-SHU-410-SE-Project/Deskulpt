import rawApis from "./raw";

type AssignWidgetId<T> = T extends (
  widgetId: string,
  ...args: infer P
) => infer R
  ? (...args: P) => R
  : never;

type WrappedApis<T> = {
  [K in keyof T]: T[K] extends Function ? AssignWidgetId<T[K]> : never;
};

function wrapApis<T>(
  widgetId: string,
  apis: T,
): { [K in keyof T]: WrappedApis<T[K]> } {
  const wrappedApis: any = {};
  for (const modName in apis) {
    wrappedApis[modName] = {};
    const module = apis[modName];
    for (const funcName in module) {
      const func = module[funcName];
      if (typeof func === "function") {
        wrappedApis[modName][funcName] = (...args: any[]) =>
          func(widgetId, ...args);
      }
    }
  }
  return wrappedApis;
}

// `__DESKULPT_WIDGET_ID__` is a placeholder; in the published package this does not
// matter at all because we only care about type hints; for internal usage this will be
// replaced with the actual widget ID
export default wrapApis("__DESKULPT_WIDGET_ID__", rawApis);
