extern crate proc_macro;

mod attributes;
mod common;
mod element;

#[proc_macro_derive(MrmlParseComponent)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::element::derive(input)
}

#[proc_macro_derive(MrmlParseAttributes)]
pub fn derive_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::attributes::derive(input)
}
