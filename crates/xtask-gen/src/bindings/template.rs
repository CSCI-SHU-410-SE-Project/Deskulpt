//! Data for the bindings template.

use anyhow::Result;
use heck::ToLowerCamelCase;
use serde::Serialize;
use specta::datatype::{DataType, Function, FunctionResultVariant};
use specta::TypeCollection;
use specta_typescript::{datatype, export_named_datatype, js_doc, Typescript};

/// Data for an event in the bindings template.
#[derive(Serialize)]
pub struct BindingsTemplateEvent {
    /// The key in the generated `events` object.
    pub key: String,
    /// The name of the event.
    pub name: String,
    /// The type representation of the event.
    pub ty: String,
}

impl BindingsTemplateEvent {
    /// Create an instance from event name and type.
    pub fn from(ts: &Typescript, tcl: &TypeCollection, name: &str, ty: &DataType) -> Result<Self> {
        Ok(Self {
            key: name.to_lower_camel_case(),
            name: name.to_string(),
            ty: datatype(ts, &FunctionResultVariant::Value(ty.clone()), tcl)?,
        })
    }
}

/// Data for an argument for a command in the bindings template.
#[derive(Serialize)]
pub struct BindingsTemplateCommandArg {
    /// The name of the argument.
    pub name: String,
    /// The type representation of the argument.
    pub ty: String,
}

impl BindingsTemplateCommandArg {
    /// Create an instance from argument name and type.
    pub fn from(ts: &Typescript, tcl: &TypeCollection, name: &str, ty: &DataType) -> Result<Self> {
        Ok(Self {
            name: name.to_lower_camel_case(),
            ty: datatype(ts, &FunctionResultVariant::Value(ty.clone()), tcl)?,
        })
    }
}

/// Data for a command in the bindings template.
#[derive(Serialize)]
pub struct BindingsTemplateCommand {
    /// The key in the generated `commands` object.
    pub key: String,
    /// The name of the command.
    pub name: String,
    /// The command arguments.
    pub args: Vec<BindingsTemplateCommandArg>,
    /// The type representation of the return value.
    pub ret_ty: String,
    /// The docstring of the command.
    pub doc: String,
}

impl BindingsTemplateCommand {
    /// Create an instance from function information.
    pub fn from(ts: &Typescript, tcl: &TypeCollection, function: &Function) -> Result<Self> {
        Ok(Self {
            key: function.name().to_lower_camel_case(),
            name: function.name().to_string(),
            args: function
                .args()
                .map(|(name, ty)| BindingsTemplateCommandArg::from(ts, tcl, name, ty))
                .collect::<Result<Vec<_>>>()?,
            ret_ty: match function.result() {
                Some(FunctionResultVariant::Value(t)) => {
                    datatype(ts, &FunctionResultVariant::Value(t.clone()), tcl)?
                },
                Some(FunctionResultVariant::Result(t, _)) => {
                    datatype(ts, &FunctionResultVariant::Value(t.clone()), tcl)?
                },
                None => "void".to_string(),
            },
            doc: {
                let mut builder = js_doc::Builder::default();
                if let Some(d) = &function.deprecated() {
                    builder.push_deprecated(d);
                }
                if !function.docs().is_empty() {
                    builder.extend(function.docs().split("\n"));
                }
                builder.build()
            },
        })
    }
}

/// Data for the bindings template.
#[derive(Serialize)]
pub struct BindingsTemplate {
    /// All types in the bindings.
    pub types: Vec<String>,
    /// All events in the bindings.
    pub events: Vec<BindingsTemplateEvent>,
    /// All commands in the bindings.
    pub commands: Vec<BindingsTemplateCommand>,
}

impl BindingsTemplate {
    /// Create an instance from export context.
    pub fn from(ts: &Typescript, cfg: &super::ExportContext) -> Result<Self> {
        Ok(Self {
            types: cfg
                .types
                .into_iter()
                .map(|(_, ndt)| Ok(export_named_datatype(ts, ndt, &cfg.types)?))
                .collect::<Result<Vec<_>>>()?,
            events: cfg
                .events
                .iter()
                .map(|(name, ty)| BindingsTemplateEvent::from(ts, &cfg.types, name, ty))
                .collect::<Result<Vec<_>>>()?,
            commands: cfg
                .commands
                .iter()
                .map(|function| BindingsTemplateCommand::from(ts, &cfg.types, function))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}
