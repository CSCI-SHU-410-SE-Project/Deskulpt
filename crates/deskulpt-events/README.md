This crate implements the events used for communication between Deskulpt backend and frontend.

Note that all event-related operations in Deskulpt should be done through the methods provided by the `EventsExt` trait on `App` and `AppHandle` instead of by manually calling Tauri event APIs. This is to ensure consistency and avoid errors.

## ⚠️ Deskulpt Internal Crate

This crate is meant to be used internally by the Deskulpt application and is not designed to support plugin authors or other users directly. Private items are documented for reference of Deskulpt developers.
