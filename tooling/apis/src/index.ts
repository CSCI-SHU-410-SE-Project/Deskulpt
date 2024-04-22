import rapis from "@deskulpt-test/raw-apis";

type ModifyFirstArg<F> = F extends (widgetId: infer W, ...args: infer P) => infer R
  ? (...args: P) => R
  : never;

type ApiWithPrependedArg<M> = {
  [Property in keyof M]: M[Property] extends (...args: any[]) => any
    ? ModifyFirstArg<M[Property]>
    : never;
};

const createApiWithWidgetId = <T>(
  widgetId: string,
  apiModule: T,
): { [P in keyof T]: ApiWithPrependedArg<T[P]> } => {
  const wrappedApis: any = {};
  for (const modName in apiModule) {
    wrappedApis[modName] = {};
    const module = apiModule[modName];
    for (const funcName in module) {
      const func = module[funcName];
      if (typeof func === "function") {
        wrappedApis[modName][funcName] = (...args: any[]) => func(widgetId, ...args);
      }
    }
  }
  return wrappedApis;
};

// Example Usage with proper typing
const apis = createApiWithWidgetId("${widgetId}", rapis);
export default apis;
