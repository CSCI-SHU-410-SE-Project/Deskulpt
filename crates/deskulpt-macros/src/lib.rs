#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/src/public/deskulpt.svg"
)]

use proc_macro::TokenStream;

mod deskulpt_event;

#[proc_macro_derive(DeskulptEvent, attributes(deskulpt))]
pub fn derive_deskulpt_event(input: TokenStream) -> TokenStream {
    deskulpt_event::proc_derive_deskulpt_event(input)
}

#[proc_macro]
pub fn register_deskulpt_events(input: TokenStream) -> TokenStream {
    deskulpt_event::proc_register_deskulpt_events(input)
}
