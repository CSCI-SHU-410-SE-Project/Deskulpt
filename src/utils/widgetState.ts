import { WidgetState } from "src/types";

/**
 * Create a blob of js code that initialize the apis for the widget.
 *
 * The js code should be like this:
 * ```
 * import initApis from "/initApis.js"
 * const apis = initApis(widgetId)
 * export default apis;
 * ```
 * @param widgetId
 */
function createWidgetApisBlob(widgetId: string) {
  const jsCode = `
import * as rapis from "@deskulpt-test/raw-apis"

function initApis(widgetId) {
	let wapis = {};
	for (const modName in rapis) {
	const mod = rapis[modName];
	const pmod = {};
	for (const funcName in mod) {
		const func = mod[funcName];
		pmod[funcName] = (...args) => func(widgetId, ...args);
	}
	wapis[modName] = pmod;
	}
	return wapis;
}

const apis = initApis("${widgetId}")
export default apis;
	`;
  return new Blob([jsCode], { type: "application/javascript" });
}

/**
 * Initialize widget state
 *
 * @param widgetId
 * @returns
 */
export function initWidgetState(widgetId: string): WidgetState {
  const blob = createWidgetApisBlob(widgetId);
  const url = URL.createObjectURL(blob);
  return {
    widgetApisBlobUrl: url,
  };
}
