import { createElement } from "react";
import { events } from "../../bindings";
import { WidgetModule, useSettings, useWidgets } from "./useStores";
import { toast } from "sonner";
import { stringifyError } from "../../utils/stringifyError";
import ErrorDisplay from "../components/ErrorDisplay";
import { useSetupEventListener } from "../../utils/useSetupEventListener";

const BASE_URL = new URL(import.meta.url).origin;
const RAW_APIS_URL = new URL("/gen/raw-apis.js", BASE_URL).href;

export function useEventListeners() {
  useUpdateSettingsListener();
  useRenderWidgetsListener();
  useShowToastListener();
}

function useUpdateSettingsListener() {
  useSetupEventListener("canvasUpdateSettings", () =>
    events.updateSettingsEvent.listen((event) => {
      useSettings.setState(() => event.payload, true);
      console.debug("Settings updated:", useSettings.getState());
    }),
  );
}

function useRenderWidgetsListener() {
  useSetupEventListener("canvasRenderWidgets", () =>
    events.renderWidgetsEvent.listen(async (event) => {
      const widgets = useWidgets.getState();

      const promises = Object.entries(event.payload).map(async ([id, code]) => {
        let apisBlobUrl;
        if (id in widgets) {
          // APIs blob URL can be reused because the contents are dependent only
          // on widget ID; the code blob URL will definitely change on re-render
          // so we revoke it here
          const widget = widgets[id]!; // We've checked id in state
          apisBlobUrl = widget.apisBlobUrl;
          if (widget.moduleBlobUrl !== undefined) {
            URL.revokeObjectURL(widget.moduleBlobUrl);
          }
        } else {
          const apisCode = window.__DESKULPT_CANVAS_INTERNALS__.apisWrapper
            .replaceAll("__DESKULPT_WIDGET_ID__", id)
            .replaceAll("__RAW_APIS_URL__", RAW_APIS_URL);
          const apisBlob = new Blob([apisCode], {
            type: "application/javascript",
          });
          apisBlobUrl = URL.createObjectURL(apisBlob);
        }

        const moduleCode = code
          .replaceAll("__DESKULPT_BASE_URL__", BASE_URL)
          .replaceAll("__DESKULPT_APIS_BLOB_URL__", apisBlobUrl);
        const moduleBlob = new Blob([moduleCode], {
          type: "application/javascript",
        });
        const moduleBlobUrl = URL.createObjectURL(moduleBlob);
        let module;
        try {
          module = await import(/* @vite-ignore */ moduleBlobUrl);
          if (module.default === undefined) {
            throw new Error("Missing default export");
          }
        } catch (error) {
          URL.revokeObjectURL(moduleBlobUrl);
          useWidgets.setState((state) => ({
            ...state,
            [id]: {
              component: () =>
                createElement(ErrorDisplay, {
                  id,
                  error: "Error importing the widget module",
                  message: stringifyError(error),
                }),
              width: "300px",
              height: "150px",
              moduleBlobUrl,
              apisBlobUrl,
            },
          }));
          return;
        }

        const widget = module.default as WidgetModule;
        useWidgets.setState((state) => ({
          ...state,
          [id]: {
            ...widget,
            moduleBlobUrl,
            apisBlobUrl,
          },
        }));
      });

      await Promise.all(promises);
    }),
  );
}

function useShowToastListener() {
  useSetupEventListener("canvasShowToast", () =>
    events.showToastEvent.listen((event) => {
      const { type, content } = event.payload;
      switch (type) {
        case "success":
          void toast.success(content);
          break;
        case "error":
          void toast.error(content);
          break;
      }
    }),
  );
}
