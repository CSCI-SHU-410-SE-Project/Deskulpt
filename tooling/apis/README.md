Note that `@deskulpt-test/raw-apis` must be included as depenednecies in `package.json` and externals in `rollup.confi.g.ts`

- depenednecies in `package.json`, because we need to infer typing of widget-specific apis from raw apis in user's widget source code folder
- externals in `rollup.confi.g.ts`, because we don't want to resolve `@deskulpt-test/raw-apis` when we build `@deskulpt-test/apis`

Also, `@deskulpt-test/apis` is only compiled into type declaration package, unlike `@deskulpt-test/raw-apis` and `@deskulpt-test/react`. This is because the widget developers only need typing support when they do `npm install @deskulpt-test/apis`.
