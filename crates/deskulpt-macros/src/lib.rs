#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/src/public/deskulpt.svg"
)]

mod persistence;

use proc_macro::TokenStream;

/// Derive the persisted version of a struct.
///
/// For `MyStruct`, this will generate a `MyStructPersisted` struct like this:
///
/// - Derived: `Default`, `Deserialize`.
/// - Struct-level `#[serde(...)]` attributes are preserved.
/// - Each field has its original type, unless overridden via `#[persisted(type
///   = "...")]`.
/// - Each field is marked with `#[serde(default)]`, unless overridden via
///   `#[persisted(default = "...")]`.
/// - Field-level `#[serde(...)]` attributes are preserved. Note that they
///   cannot contain `default`.
/// - A `From<MyStructPersisted>` implementation for `MyStruct`.
/// - A `FromPersisted<MyStructPersisted>` implementation for `MyStruct`.
#[proc_macro_derive(Persisted, attributes(persisted))]
pub fn derive_persisted(input: TokenStream) -> TokenStream {
    persistence::proc_derive_persisted(input)
}
