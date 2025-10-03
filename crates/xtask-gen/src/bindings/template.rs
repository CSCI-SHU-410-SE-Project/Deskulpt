use anyhow::Result;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use heck::ToLowerCamelCase;
use regex::Regex;
use serde::Serialize;
use specta::datatype::{DataType, Function, FunctionResultVariant};
use specta::TypeCollection;
use specta_typescript::{datatype, export_named_datatype, js_doc, Typescript};

/// Similar to [`export_named_datatype`] but for [`DataType`].
fn export_datatype(ts: &Typescript, typ: &DataType, tcl: &TypeCollection) -> Result<String> {
    Ok(datatype(
        ts,
        &FunctionResultVariant::Value(typ.clone()),
        tcl,
    )?)
}

/// Template data for an event.
#[derive(Serialize)]
struct EventTemplate {
    key: String,
    name: String,
    ty: String,
}

impl EventTemplate {
    fn from(ts: &Typescript, tcl: &TypeCollection, name: &str, ty: &DataType) -> Result<Self> {
        Ok(Self {
            key: name.to_lower_camel_case(),
            name: name.to_string(),
            ty: export_datatype(ts, ty, tcl)?,
        })
    }
}

/// Template data for a command argument.
#[derive(Serialize)]
struct CommandArgTemplate {
    name: String,
    ty: String,
}

impl CommandArgTemplate {
    fn from(ts: &Typescript, tcl: &TypeCollection, name: &str, ty: &DataType) -> Result<Self> {
        Ok(Self {
            name: name.to_lower_camel_case(),
            ty: export_datatype(ts, ty, tcl)?,
        })
    }
}

/// Template data for a command.
#[derive(Serialize)]
struct CommandTemplate {
    key: String,
    name: String,
    args: Vec<CommandArgTemplate>,
    ret_ty: String,
    doc: String,
}

impl CommandTemplate {
    fn from(ts: &Typescript, tcl: &TypeCollection, function: &Function) -> Result<Self> {
        Ok(Self {
            key: function.name().to_lower_camel_case(),
            name: function.name().to_string(),
            args: function
                .args()
                .map(|(name, ty)| CommandArgTemplate::from(ts, tcl, name, ty))
                .collect::<Result<Vec<_>>>()?,
            ret_ty: match function.result() {
                Some(FunctionResultVariant::Value(t))
                | Some(FunctionResultVariant::Result(t, _)) => export_datatype(ts, t, tcl)?,
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

/// Full template data.
#[derive(Serialize)]
struct Template {
    types: Vec<String>,
    events: Vec<EventTemplate>,
    commands: Vec<CommandTemplate>,
}

impl Template {
    fn from(ts: Typescript, builder: super::Builder) -> Result<Self> {
        Ok(Self {
            types: builder
                .types
                .into_iter()
                .map(|(_, ndt)| Ok(export_named_datatype(&ts, ndt, &builder.types)?))
                .collect::<Result<Vec<_>>>()?,
            events: builder
                .events
                .iter()
                .map(|(name, ty)| EventTemplate::from(&ts, &builder.types, name, ty))
                .collect::<Result<Vec<_>>>()?,
            commands: builder
                .commands
                .iter()
                .map(|function| CommandTemplate::from(&ts, &builder.types, function))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

/// Handlebars helper to indent (multi-line) text by a given number of spaces.
fn handlebars_indent_helper(
    h: &Helper<'_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    let text = h
        .param(0)
        .and_then(|p| p.value().as_str())
        .unwrap_or_default();
    let spaces = h
        .param(1)
        .and_then(|p| p.value().as_u64())
        .unwrap_or_default();

    let pad = " ".repeat(spaces as usize);
    for (i, line) in text.lines().enumerate() {
        if i > 0 {
            out.write("\n")?;
        }
        out.write(&pad)?;
        out.write(line)?;
    }

    Ok(())
}

/// Render the bindings template.
pub fn render(builder: super::Builder) -> Result<String> {
    let mut hb = Handlebars::new();
    hb.register_escape_fn(handlebars::no_escape);
    hb.register_helper("indent", Box::new(handlebars_indent_helper));
    hb.register_template_string("bindings", include_str!("template.ts.hbs"))?;

    let data = Template::from(Typescript::new(), builder)?;
    let output = hb.render("bindings", &data)?;

    // TODO: Remove when specta > 2.0.0-rc.22
    let re = Regex::new(r"Partial\s*<\s*(\{\s*\[\s*key\s+in\s+string\s*\][^}]*\})\s*>").unwrap();
    let output = re.replace_all(&output, "$1").to_string();

    Ok(output)
}
