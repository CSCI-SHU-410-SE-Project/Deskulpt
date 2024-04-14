// mod apis;
// pub(crate) mod utils;
//
// use tauri::{
//     plugin::{Builder, TauriPlugin},
//     Runtime,
// };
//
// /// The registered APIs can be invoked as `plugin:widget_api.fs|<api_name>``.
// pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
//     Builder::new("widget_api.fs")
//         .invoke_handler(tauri::generate_handler![
//             apis::is_file,
//             apis::is_dir,
//             apis::exists,
//             apis::read_file,
//             apis::write_file,
//             apis::append_file,
//             apis::remove_file,
//             apis::create_dir,
//             apis::remove_dir,
//         ])
//         .build()
// }
