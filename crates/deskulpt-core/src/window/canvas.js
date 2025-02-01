Object.defineProperty(window, "__DESKULPT_CANVAS_INTERNALS__", {
  value: {
    apisWrapper: __TEMPLATE_apis_wrapper__,
  },
  writable: false,
  configurable: false,
  enumerable: false,
});

const props = window.__DESKULPT_CANVAS_INTERNALS__;
Object.freeze(props);
