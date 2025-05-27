//! Internals of the `#[dispatch]` macro.

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, FnArg, ItemFn, Pat, PatType, ReturnType};

/// Token stream processor for the `#[dispatch]` macro.
///
/// This clones the AST of the original function and performs the following
/// modifications:
///
/// - Get the type of the `input` parameter of the function and replace it with
///   `&[u8]`. Not having an `input` parameter panics.
/// - Get the return type of the function and replace it with
///   `anyhow::Result<Vec<u8>>`. Not specifying an explicit return type panics.
/// - Wrap the original function body in a block that deserializes the input,
///   calls the original function, serializes the output, and returns it. Note
///   that the original function must have a return type that the `?` operator
///   can be applied to.
pub fn proc_dispatch(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut meth = parse_macro_input!(item as ItemFn);

    let mut input_type = None;
    for arg in &mut meth.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let Pat::Ident(ident) = &**pat {
                if ident.ident == "input" {
                    input_type = Some(ty.clone());
                    *ty = Box::new(parse_quote!(&[u8]));
                }
            }
        }
    }
    let input_type = input_type.expect("Missing `input` parameter");

    let output_type = if let ReturnType::Type(_, ty) = meth.sig.output {
        ty
    } else {
        panic!("Return type must be specified");
    };
    meth.sig.output = parse_quote!(-> ::deskulpt_plugin::anyhow::Result<Vec<u8>>);

    let original_body = meth.block.clone();
    meth.block = Box::new(parse_quote!({
        let input: #input_type = ::deskulpt_plugin::anyhow::Context::context(::deskulpt_plugin::rmp_serde::from_slice(input), "Failed to deserialize input")?;
        let result: #output_type = #original_body;
        let result = result?;
        let output = ::deskulpt_plugin::anyhow::Context::context(::deskulpt_plugin::rmp_serde::to_vec_named(&result), "Failed to serialize output")?;
        Ok(output)
    }));

    meth.into_token_stream().into()
}
