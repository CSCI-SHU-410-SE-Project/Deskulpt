//! This file defines a custom Tauri plugin for the widget API.
//!
//! We use a tauri plugin instead of directly registering tauri commands because
//! this allow us to invoke commands with namespace, e.g.
//! `invoke("plugin:widget-sys.fs|read_file", ...)`.

pub(crate) mod sys;
