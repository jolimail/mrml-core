extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, Path, Type, TypePath,
};

fn as_fields_named(input: &DataStruct) -> Option<&FieldsNamed> {
    if let Fields::Named(inner) = &input.fields {
        Some(inner)
    } else {
        None
    }
}

fn as_data_struct(ast: &DeriveInput) -> Option<&DataStruct> {
    if let Data::Struct(inner) = &(ast.data) {
        Some(inner)
    } else {
        None
    }
}

fn get_fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
    as_data_struct(ast)
        .and_then(as_fields_named)
        .map(|f| &f.named)
        .expect("MrmlPrintComponent only supports structs.")
}

fn get_attributes_field(ast: &DeriveInput) -> Option<&Field> {
    get_fields(ast).into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|id| id.to_string() == "attributes")
            .unwrap_or(false)
    })
}

fn get_children_field(ast: &DeriveInput) -> Option<&Field> {
    as_data_struct(ast)
        .and_then(as_fields_named)
        .map(|f| &f.named)
        .expect("MrmlPrintComponent only supports structs.")
        .into_iter()
        .find(|f| {
            f.ident
                .as_ref()
                .map(|id| id.to_string() == "children")
                .unwrap_or(false)
        })
}

#[derive(FromDeriveInput)]
#[darling(attributes(mrml_print), forward_attrs(allow, doc, cfg))]
struct Opts {
    tag: Option<String>,
    close_empty: Option<bool>,
    indent_children: Option<bool>,
}

impl Opts {
    fn indent_children(&self) -> bool {
        self.indent_children.unwrap_or(true)
    }
}

fn is_map_string(path: &Path) -> bool {
    path.segments
        .first()
        // TODO make sure that it's a Map<String, String>
        .map(|s| s.ident == "Map")
        .unwrap_or(false)
}

fn print_attributes(ast: &DeriveInput) -> proc_macro2::TokenStream {
    if let Some(field) = get_attributes_field(ast) {
        match &field.ty {
            Type::Path(TypePath { path, .. }) if is_map_string(path) => {
                quote! { Some(&self.attributes) }
            }
            _ => {
                quote! { Some(&self.attributes.as_map()) }
            }
        }
    } else {
        quote! { None }
    }
}

#[derive(PartialEq, Eq)]
enum ChildrenKind {
    String { indent: bool },
    List,
    None,
}

fn get_children_kind(ast: &DeriveInput, opts: &Opts) -> ChildrenKind {
    if let Some(field) = get_children_field(ast) {
        match &field.ty {
            Type::Path(TypePath { path, .. }) if path.is_ident("String") => ChildrenKind::String {
                indent: opts.indent_children(),
            },
            _ => ChildrenKind::List,
        }
    } else {
        ChildrenKind::None
    }
}

fn impl_print(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let opts = Opts::from_derive_input(&ast).expect("Wrong options");

    let tag_name = opts.tag.clone().unwrap_or_else(|| "NAME".into());
    let tag_name = Ident::new(tag_name.as_str(), Span::call_site());

    let attrs = print_attributes(ast);

    let printing = match get_children_kind(ast, &opts) {
        ChildrenKind::None => {
            let close_empty = opts.close_empty.unwrap_or(true);
            quote! {
                crate::prelude::print::open(#tag_name, #attrs, #close_empty, pretty, level, indent_size)
            }
        }
        ChildrenKind::String { indent: true } => {
            quote! {
                if self.children.is_empty() {
                    crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
                } else {
                    let mut res = crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size);
                    res.push_str(&self.children);
                    res.push_str(&crate::prelude::print::close(#tag_name, pretty, level, indent_size));
                    res
                }
            }
        }
        ChildrenKind::String { indent: false } => {
            quote! {
                if self.children.is_empty() {
                    crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
                } else {
                    let mut res = crate::prelude::print::open(#tag_name, #attrs, false, false, level, indent_size);
                    res.push_str(&self.children);
                    res.push_str(&crate::prelude::print::close(#tag_name, false, level, indent_size));
                    if pretty {
                        crate::prelude::print::indent(level, indent_size, res)
                    } else {
                        res
                    }
                }
            }
        }
        ChildrenKind::List => {
            quote! {
                if self.children.is_empty() {
                    crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
                } else {
                    let mut res = crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size);
                    for child in self.children.iter() {
                        res.push_str(&child.print(pretty, level + 1, indent_size));
                    }
                    res.push_str(&crate::prelude::print::close(#tag_name, pretty, level, indent_size));
                    res
                }
            }
        }
    };

    // let printing = if let Some(children_field) = children_field {
    //     match &children_field.ty {
    //         Type::Path(TypePath { path, .. }) if path.is_ident("String") => {
    //             quote! {
    //                 if self.children.is_empty() {
    //                     crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
    //                 } else if pretty {
    //                     crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size)
    //                         + &self.children + "\n"
    //                         + &crate::prelude::print::close(#tag_name, pretty, level, indent_size)
    //                 } else {
    //                     crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size)
    //                         + &self.children
    //                         + &crate::prelude::print::close(#tag_name, pretty, level, indent_size)
    //                 }
    //             }
    //         }
    //         _ => {
    //             quote! {
    //                 if self.children.is_empty() {
    //                     crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
    //                 } else {
    //                     crate::prelude::print::open(
    //                         #tag_name,
    //                         #attrs,
    //                         false,
    //                         pretty,
    //                         level,
    //                         indent_size,
    //                     ) + &self
    //                         .children
    //                         .iter()
    //                         .map(|child| child.print(pretty, level + 1, indent_size))
    //                         .collect::<String>()
    //                         + &crate::prelude::print::close(#tag_name, pretty, level, indent_size)
    //                 }
    //             }
    //         }
    //     }
    // } else {
    //     if opts.close_empty.unwrap_or(true) {
    //         quote! {
    //             crate::prelude::print::open(#tag_name, #attrs, true, pretty, level, indent_size)
    //         }
    //     } else {
    //         quote! {
    //             crate::prelude::print::open(#tag_name, #attrs, false, pretty, level, indent_size)
    //         }
    //     }
    // };

    quote! {
        impl crate::prelude::print::Print for #name {
            fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
                #printing
            }
        }
    }
}

fn impl_display(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use crate::prelude::print::Print;

                f.write_str(self.dense_print().as_str())
            }
        }
    }
}

#[proc_macro_derive(MrmlPrintComponent, attributes(mrml_print))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let print_impl = impl_print(&ast);
    let display_impl = impl_display(&ast);

    quote! {
        #print_impl
        #display_impl
    }
    .into()
}

#[proc_macro_derive(MrmlPrintAttributes)]
pub fn derive_attributes(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = get_fields(&ast).iter().filter_map(|f| f.ident.as_ref());

    let res = quote! {
        impl #name {
            fn as_map(&self) -> crate::prelude::hash::Map<String, String> {
                let mut res = crate::prelude::hash::Map::new();
                #(
                    res.insert(stringify!(#fields).to_string(), self.#fields.to_string());
                )*
                res
            }
        }
    };

    res.into()
}
