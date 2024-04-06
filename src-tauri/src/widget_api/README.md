# Implementation of Widget API

The name of a tauri command is closely tied to the function name. It can neither be renamed or be associated with a nested namespace. To organize different types of widget apis into different workspace, a workaround is to use multiple plugins with namespaced, different names.

For example, let's say we want a set of widget apis for file system access, and another set of apis for getting system information. We can create two plugins, `widget_api.fs` and `widget_api.sysinfo`. The folder structure will look like this:

```
src-tauri/
  src/
	widget_api/
	  fs/
	    apis.rs
		mod.rs
	  sysinfo/
	    apis.rs
		mod.rs
	  mod.rs
```

- `widget_api/fs/apis.rs` will contain the file system apis
- `widget_api/fs/mod.rs` expose the `init` function that initialize the `widget_api.fs` plugin
- `widget_api/sysinfo/apis.rs` will contain the system information apis
- `widget_api/sysinfo/mod.rs` expose the `init` function that initialize the `widget_api.sysinfo` plugin
- `widget_api/mod.rs` expose the `fs` and `sysinfo` modules to `main.rs` so that they can be initialized

Note that `widget_api` module itself doesn't define any tauri plugin. It only serves to expose the plugins defined in the submodules. The common namespace `widget_api` of ``widget_api.fs` and `widget_api.sysinfo` is not enforced by tauri, but only an illusion provided by the freedom to name plugins. Deskulpt developers need to actively maintain the namespace consistency. That's why this is merely a workaround.
