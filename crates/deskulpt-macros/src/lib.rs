#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/public/deskulpt.svg"
)]

mod event;

use proc_macro::TokenStream;

/// Derive the `Event` trait for a struct.
///
/// The name of the struct must end with `Event`, and the event name will be
/// the struct name without the `Event` suffix, converted to kebab-case.
#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    event::proc_derive_event(input)
}
