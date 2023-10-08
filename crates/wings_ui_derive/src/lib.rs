extern crate proc_macro;

use proc_macro::TokenStream;

mod custom_syntax;

#[proc_macro]
pub fn ui_builder(input: TokenStream) -> TokenStream {
    custom_syntax::ui_builder(input.into()).into()
}
