#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/src/public/deskulpt.svg",
    html_favicon_url = "https://github.com/CSCI-SHU-410-SE-Project/Deskulpt/raw/main/packages/deskulpt/src/public/deskulpt.svg"
)]

use proc_macro::TokenStream;

mod dispatch;

#[proc_macro_attribute]
pub fn dispatch(attr: TokenStream, item: TokenStream) -> TokenStream {
    dispatch::proc_dispatch(attr, item)
}
