/**
 * The state of the widget. This includes
 * - the blob url of the javascript bundle for widget-specific APIs (raw APIS with widgetId passed in)
 * - TODO: style of widgets: widget position, size, etc.
 */
export interface WidgetState {
  widgetApisBlobUrl: string;
}
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
async function createWidgetApisBlob(widgetId: string) {
  // "/apis.txt" is compiled from the npm package in /tooling/apis
  // It is a template for the widget's apis
  const jsCodePath = "/apis.txt";
  const jsCodeResponse = await fetch(jsCodePath);
  const jsCodeTemplate = await jsCodeResponse.text();
  console.log(`jsCodeTemplate: ${jsCodeTemplate}`);
  const jsCode = jsCodeTemplate.replace("${widgetId}", widgetId);
  console.log(`jsCode: ${jsCode}`);

  return new Blob([jsCode], { type: "application/javascript" });
}

/**
 * Initialize widget state
 *
 * @param widgetId
 * @returns
 */
export async function initWidgetState(widgetId: string): Promise<WidgetState> {
  const blob = await createWidgetApisBlob(widgetId);
  const url = URL.createObjectURL(blob);
  return {
    widgetApisBlobUrl: url,
  };
}
