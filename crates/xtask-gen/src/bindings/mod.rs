mod helpers;
mod template;

use std::collections::BTreeMap;

use anyhow::Result;
use deskulpt_core::events::Event;
use handlebars::Handlebars;
use regex::Regex;
use specta::datatype::{DataType, Function};
use specta::function::collect_functions;
use specta::{NamedType, Type, TypeCollection};

pub struct ExportContext {
    pub commands: Vec<Function>,
    pub events: BTreeMap<&'static str, DataType>,
    pub types: TypeCollection,
}

pub fn run() -> Result<()> {
    let mut types = TypeCollection::default();

    let commands = collect_functions![
        deskulpt_core::commands::bundle_widget::<tauri::Wry>,
        deskulpt_core::commands::call_plugin::<tauri::Wry>,
        deskulpt_core::commands::emit_on_render_ready::<tauri::Wry>,
        deskulpt_core::commands::exit_app::<tauri::Wry>,
        deskulpt_core::commands::open_widget::<tauri::Wry>,
        deskulpt_core::commands::rescan_widgets::<tauri::Wry>,
        deskulpt_core::commands::set_render_ready::<tauri::Wry>,
        deskulpt_core::commands::update_settings::<tauri::Wry>,
    ](&mut types);

    let mut events = BTreeMap::new();

    macro_rules! add_event {
        ($ty:path) => {
            let name = <$ty as Event>::NAME;
            let dt = <$ty as Type>::reference(&mut types, &[]).inner;
            events.insert(name, dt);
        };
    }

    add_event!(deskulpt_core::events::ExitAppEvent);
    add_event!(deskulpt_core::events::RemoveWidgetsEvent);
    add_event!(deskulpt_core::events::RenderWidgetsEvent);
    add_event!(deskulpt_core::events::ShowToastEvent);
    add_event!(deskulpt_core::events::SwitchThemeEvent);
    add_event!(deskulpt_core::events::UpdateSettingsEvent);

    {
        type T = deskulpt_core::window::DeskulptWindow;
        let dt = <T as NamedType>::definition_named_data_type(&mut types);
        types.insert(<T as NamedType>::sid(), dt);
    }

    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);
    handlebars.register_helper("indent", Box::new(helpers::indent));
    handlebars.register_template_string("bindings", include_str!("template.ts.hbs"))?;

    let ctx = ExportContext {
        commands,
        events,
        types,
    };
    let data = template::BindingsTemplate::from(&specta_typescript::Typescript::new(), &ctx)?;
    let rendered = handlebars.render("bindings", &data)?;

    // TODO: Remove when specta > 2.0.0-rc.22
    let re = Regex::new(r"Partial\s*<\s*(\{\s*\[\s*key\s+in\s+string\s*\][^}]*\})\s*>").unwrap();
    let rendered = re.replace_all(&rendered, "$1").to_string();

    let path = deskulpt_workspace::package_dir("deskulpt").join("src/bindings.ts");
    std::fs::write(&path, rendered)?;
    Ok(())
}
