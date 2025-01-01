This crate implements internal state management of the Deskulpt application.

Note that all state management in Deskulpt should be done through the methods provided by the `StatesExt` trait on `App` and `AppHandle` instead of by manually calling Tauri state management APIs. This is to ensure consistency and avoid errors.

## ⚠️ Deskulpt Internal Crate

This crate is meant to be used internally by the Deskulpt application and is not designed to support plugin authors or other users directly. Private items are documented for reference of Deskulpt developers.
