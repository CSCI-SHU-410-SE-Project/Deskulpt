mod template;

use std::collections::BTreeMap;

use anyhow::Result;
use deskulpt_core::events::Event;
use specta::datatype::{DataType, Function};
use specta::function::collect_functions;
use specta::{NamedType, Type, TypeCollection};

#[derive(Default)]
struct Builder {
    types: TypeCollection,
    events: BTreeMap<&'static str, DataType>,
    commands: Vec<Function>,
}

impl Builder {
    fn typ<T: NamedType>(mut self) -> Self {
        let dt = T::definition_named_data_type(&mut self.types);
        self.types.insert(T::sid(), dt);
        self
    }

    fn event<T: Event + Type>(mut self) -> Self {
        let dt = T::reference(&mut self.types, &[]).inner;
        self.events.insert(T::NAME, dt);
        self
    }

    fn commands(mut self, commands: fn(&mut TypeCollection) -> Vec<Function>) -> Self {
        self.commands = commands(&mut self.types);
        self
    }
}

/// Entry point for the `cargo gen bindings` command.
pub fn run() -> Result<()> {
    let builder = Builder::default()
        .commands(collect_functions![
            deskulpt_core::commands::bundle_widget::<tauri::Wry>,
            deskulpt_core::commands::call_plugin::<tauri::Wry>,
            deskulpt_core::commands::emit_on_render_ready::<tauri::Wry>,
            deskulpt_core::commands::exit_app::<tauri::Wry>,
            deskulpt_core::commands::open_widget::<tauri::Wry>,
            deskulpt_core::commands::rescan_widgets::<tauri::Wry>,
            deskulpt_core::commands::set_render_ready::<tauri::Wry>,
            deskulpt_core::commands::update_settings::<tauri::Wry>,
        ])
        .event::<deskulpt_core::events::ExitAppEvent>()
        .event::<deskulpt_core::events::RemoveWidgetsEvent>()
        .event::<deskulpt_core::events::RenderWidgetsEvent>()
        .event::<deskulpt_core::events::ShowToastEvent>()
        .event::<deskulpt_core::events::SwitchThemeEvent>()
        .event::<deskulpt_core::events::UpdateSettingsEvent>()
        .typ::<deskulpt_core::window::DeskulptWindow>();

    let output = template::render(builder)?;
    let path = deskulpt_workspace::package_dir("deskulpt").join("src/bindings.ts");
    std::fs::write(&path, output)?;

    Ok(())
}
