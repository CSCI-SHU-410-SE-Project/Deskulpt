/** @type {import("typedoc").TypeDocOptions} */

export default {
  name: "Deskulpt TypeScript Doc",
  out: "./website/static/tsdoc",
  entryPoints: ["./src"],
  entryPointStrategy: "expand",
};
