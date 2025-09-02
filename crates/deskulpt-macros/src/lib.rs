#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

mod persistence;

use proc_macro::TokenStream;

/// Derive the persisted version of a struct.
///
/// For `MyStruct`, this will generated a deserializable `MyStructPersisted`
/// struct with each field marked with `#[serde(default)]`. If a field needs a
/// custom default, it can be specified via `#[persisted(default = "...")]`
/// which has the same semantic as `#[serde(default = "...")]`.
///
/// By default each field will be persisted with its original type, but this can
/// be overridden via `#[persisted(type = "...")]`. This is useful for nested
/// persisted structs.
///
/// This macro will further implement `From<MyStructPersisted>` and
/// `FromPersisted<MyStructPersisted>` for `MyStruct`. The former is so that a
/// `MyStructPersisted` loaded from persistence can be easily converted to
/// `MyStruct` for use in the application. The latter is helpful for generating
/// automatic conversion implementations.
#[proc_macro_derive(Persisted, attributes(persisted))]
pub fn derive_persisted(input: TokenStream) -> TokenStream {
    persistence::proc_derive_persisted(input)
}
