extern crate proc_macro;

mod element;

use common_macros::{as_data_enum, as_data_struct, as_path, get_fields, is_option};
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataStruct, DeriveInput};

#[proc_macro_derive(MrmlPrintComponent, attributes(mrml_print))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = element::Opts::from_derive_input(&ast).expect("Wrong options");

    element::Generator::from((ast, opts)).build().into()
}

#[proc_macro_derive(MrmlPrintAttributes)]
pub fn derive_attributes(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields =
        get_fields(&ast)
            .iter()
            .filter_map(|f| match (&f.ident, as_path(f).map(is_option)) {
                (Some(ident), Some(true)) => Some(quote! {
                    if let Some(ref value) = self.#ident {
                        res.insert(stringify!(#ident).to_string(), value.to_string());
                    }
                }),
                (Some(ident), Some(false)) => Some(quote! {
                    res.insert(stringify!(#ident).to_string(), self.#ident.to_string());
                }),
                _ => None,
            });

    let res = quote! {
        impl #name {
            fn as_map(&self) -> crate::prelude::hash::Map<String, String> {
                let mut res = crate::prelude::hash::Map::new();
                #(#fields)*
                res
            }
        }
    };

    res.into()
}

#[proc_macro_derive(MrmlPrintChildren)]
pub fn derive_children(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    if let Some(data_enum) = as_data_enum(&ast) {
        derive_children_enum(&ast, data_enum).into()
    } else if let Some(data_struct) = as_data_struct(&ast) {
        derive_children_struct(&ast, data_struct).into()
    } else {
        panic!("MrmlPrintChildren only works with enums and structs.")
    }
}

fn derive_children_enum(ast: &DeriveInput, data_enum: &DataEnum) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let fields = data_enum
        .variants
        .iter()
        .map(|v| {
            let variant = &v.ident;
            quote! {
                #name::#variant(elt) => elt.print(pretty, level, indent_size),
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        impl crate::prelude::print::Print for #name {
            fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                match self {
                    #fields
                }
            }
        }
    }
}

fn derive_children_struct(ast: &DeriveInput, data_struct: &DataStruct) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let fields =
        data_struct
            .fields
            .iter()
            .filter_map(|f| match (&f.ident, as_path(f).map(is_option)) {
                (Some(ident), Some(true)) => Some(quote! {
                    if let Some(ref value) = self.#ident {
                        res.push_str(&value.print(pretty, level, indent_size));
                    }
                }),
                (Some(ident), Some(false)) => Some(quote! {
                    res.push_str(&self.#ident.print(pretty, level, indent_size));
                }),
                _ => None,
            });

    quote! {
        impl crate::prelude::print::Print for #name {
            fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                let mut res = String::new();
                #(#fields)*
                res
            }
        }
    }
}
