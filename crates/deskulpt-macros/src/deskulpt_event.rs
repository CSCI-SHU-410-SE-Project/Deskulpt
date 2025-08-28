use heck::ToKebabCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Ident, Lit, Token};

pub fn proc_derive_deskulpt_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    let mut override_name = None::<String>;
    for attr in &input.attrs {
        if attr.path().is_ident("deskulpt_event") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    // #[deskulpt_event(name="xxx")] overrides the event name
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(s) = value {
                        override_name = Some(s.value());
                    }
                }
                Ok(())
            });
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let raw = ident.to_string();
    let trimmed = raw.strip_suffix("Event").unwrap_or(&raw).to_string();
    let name_literal = override_name.unwrap_or_else(|| trimmed.to_kebab_case());

    let expanded = quote! {
        impl #impl_generics crate::events::DeskulptEvent for #ident #ty_generics #where_clause {
            const NAME: &'static str = #name_literal;
        }
    };
    TokenStream::from(expanded)
}

pub fn proc_register_deskulpt_events(input: TokenStream) -> TokenStream {
    let event_types =
        parse_macro_input!(input with Punctuated::<Ident, Token![,]>::parse_terminated);

    let event_info = event_types.iter().map(|event_type| {
        let event_type_name = event_type.to_string();
        quote! { (#event_type_name, #event_type::NAME) }
    });

    let expanded = quote! {
        #[cfg(test)]
        #[test]
        fn export_bindings_deskulpt_events() {
            let export_dir = std::env::var("TS_RS_EXPORT_DIR").unwrap_or("./bindings".into());
            let output_path = std::path::Path::new(&export_dir).join("events.ts");

            let mut content =
                "/*! Auto-generated bindings for Deskulpt events. DO NOT EDIT! */\n\n".to_string();
            content.push_str("import { __makeEventAPI__ } from \"../bindingsHelper\";\n\n");

            let event_info = vec![#(#event_info),*];
            for (event_type, event_name) in event_info {
                content.push_str(&format!("import {{ {} }} from \"./types\";\n", event_type));
                content.push_str(&format!(
                    "export const {}API = __makeEventAPI__<{}>(\"{}\");\n\n",
                    event_type, event_type, event_name
                ));
            }

            if let Some(parent) = output_path.parent() {
                std::fs::create_dir_all(parent).expect("Failed to create output directory");
            }
            std::fs::write(&output_path, content).expect("Failed to write the bindings");
        }
    };

    TokenStream::from(expanded)
}
