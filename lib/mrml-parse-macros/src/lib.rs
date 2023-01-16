extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, Ident, Type, TypePath,
};

// fn is_vec(path: &Path) -> bool {
//     path.segments
//         .first()
//         // TODO make sure that it's a Vec<T>
//         .map(|s| s.ident == "Vec")
//         .unwrap_or(false)
// }

fn as_data_struct(ast: &DeriveInput) -> Option<&DataStruct> {
    if let Data::Struct(inner) = &(ast.data) {
        Some(inner)
    } else {
        None
    }
}

fn as_fields_named(input: &DataStruct) -> Option<&FieldsNamed> {
    if let Fields::Named(inner) = &input.fields {
        Some(inner)
    } else {
        None
    }
}

fn get_fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
    as_data_struct(ast)
        .and_then(as_fields_named)
        .map(|f| &f.named)
        .expect("MrmlParseComponent only supports structs.")
}

fn get_children_field(ast: &DeriveInput) -> Option<&Field> {
    get_fields(ast).into_iter().find(|f| {
        f.ident
            .as_ref()
            .map(|id| *id == "children")
            .unwrap_or(false)
    })
}

enum ChildrenKind {
    String,
    List,
    None,
}

fn get_children_kind(ast: &DeriveInput) -> ChildrenKind {
    if let Some(field) = get_children_field(ast) {
        match &field.ty {
            Type::Path(TypePath { path, .. }) if path.is_ident("String") => ChildrenKind::String,
            _ => ChildrenKind::List,
        }
    } else {
        ChildrenKind::None
    }
}

#[proc_macro_derive(MrmlParseComponent)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let origin_ident = &ast.ident;
    let parser_name = format!("{origin_ident}Parser");
    let parser_ident = Ident::new(&parser_name, origin_ident.span());

    let parse_child_text = match get_children_kind(&ast) {
        ChildrenKind::None => quote! {},
        ChildrenKind::String => quote! {
            fn parse_child_text(&mut self, value: xmlparser::StrSpan) -> Result<(), crate::prelude::parse::Error> {
                self.element.children = value.to_string();
                Ok(())
            }
        },
        ChildrenKind::List => quote! {},
    };

    quote! {
        #[derive(Debug)]
        struct #parser_ident {
            element: #origin_ident,
        }

        impl #parser_ident {
            fn new() -> Self {
                Self {
                    element: <#origin_ident>::default(),
                }
            }
        }

        impl crate::prelude::parse::Parser for #parser_ident {
            type Output = #origin_ident;

            fn build(self) -> Result<Self::Output, crate::prelude::parse::Error> {
                Ok(self.element)
            }

            #parse_child_text
        }

        impl crate::prelude::parse::Parsable for #origin_ident {
            fn parse(_tag: xmlparser::StrSpan, tokenizer: &mut xmlparser::Tokenizer) -> Result<Self, crate::prelude::parse::Error> {
                use crate::prelude::parse::Parser;
                #parser_ident::new().parse(tokenizer)?.build()
            }
        }
    }
    .into()
}
