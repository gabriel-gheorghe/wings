extern crate proc_macro;

use proc_macro::TokenStream;

mod widget_tree;

#[proc_macro]
pub fn widget_tree(input: TokenStream) -> TokenStream {
    widget_tree::widget_tree(input.into()).into()
}
