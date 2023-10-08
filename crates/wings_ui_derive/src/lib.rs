extern crate proc_macro;

use proc_macro::TokenStream;

mod custom_syntax;

#[proc_macro]
pub fn widget_tree(input: TokenStream) -> TokenStream {
    custom_syntax::widget_tree(input.into()).into()
}
