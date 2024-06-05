import { ManagerWidgetState } from "../types/frontend";
import { IdMap } from "../types/backend";
import { emitRenderWidgetToCanvas } from "../events";

/**
 * Notify the canvas to render a collection of widgets.
 *
 * The event is emitted for each widget asynchronously in parallel.
 */
export async function renderWidgets(managerWidgetStates: IdMap<ManagerWidgetState>) {
  await Promise.all(
    Object.entries(managerWidgetStates).map(([widgetId, { setting }]) =>
      emitRenderWidgetToCanvas({ widgetId, setting, bundle: true }),
    ),
  );
}
