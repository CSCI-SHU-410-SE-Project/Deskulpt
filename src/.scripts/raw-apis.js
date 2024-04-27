import { invoke as t } from "@tauri-apps/api";

var e = {
  fs: Object.freeze({
    __proto__: null, exists: function(e, i) {
      return t("plugin:widget_api.fs|exists", { widgetId: e, path: i });
    }, isFile: function(e, i) {
      return t("plugin:widget_api.fs|is_file", { widgetId: e, path: i });
    }, readFile: function(e, i) {
      return t("plugin:widget_api.fs|read_file", { widgetId: e, path: i });
    }, writeFile: function(e, i, n) {
      return t("plugin:widget_api.fs|write_file", { widgetId: e, path: i, content: n });
    }
  }), sys: Object.freeze({
    __proto__: null, getSystemInfo: function(e, i) {
      return t("plugin:widget_api.sys|get_system_info", { widgetId: e, path: i });
    }
  })
};
export { e as default };
