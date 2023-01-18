use darling::FromDeriveInput;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(attributes(mrml_json), forward_attrs(allow, doc, cfg))]
struct Opts {
    tag: String,
}

fn create_serializer(ast: &DeriveInput, opts: &Opts) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;

    let element_name = &opts.tag;
    let element_ident = syn::Ident::new(element_name, struct_ident.span());

    let mut fields: usize = 1;

    let has_attributes = common_macros::get_attributes_field(ast).is_some();
    let attributes = if has_attributes {
        fields += 1;
        quote! {
            if !self.attributes.is_empty() {
                map.serialize_entry("attributes", &self.attributes)?;
            }
        }
    } else {
        quote! {}
    };

    let has_children = common_macros::get_children_field(ast).is_some();
    let children = if has_children {
        fields += 1;
        quote! {
            if !self.children.is_empty() {
                map.serialize_entry("children", &self.children)?;
            }
        }
    } else {
        quote! {}
    };

    quote! {
        use serde::ser::SerializeMap;

        impl serde::Serialize for #struct_ident {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(#fields))?;
                map.serialize_entry("type", #element_ident)?;
                #attributes
                #children
                map.end()
            }
        }
    }
}

fn create_deserialize(ast: &DeriveInput, opts: &Opts) -> proc_macro2::TokenStream {
    let struct_ident = &ast.ident;
    let element_name = &opts.tag;
    let element_ident = syn::Ident::new(element_name, struct_ident.span());
    let visitor_ident = syn::Ident::new(&format!("{struct_ident}Visitor"), struct_ident.span());

    let has_attributes = common_macros::get_attributes_field(ast).is_some();
    let has_children = common_macros::get_children_field(ast).is_some();

    let set_attributes = if has_attributes {
        quote! {
            else if key == "attributes" {
                result.attributes = access.next_value()?;
            }
        }
    } else {
        quote! {}
    };
    let set_children = if has_children {
        quote! {
            else if key == "children" {
                result.children = access.next_value()?;
            }
        }
    } else {
        quote! {}
    };

    let (fields, formatter) = if has_attributes && has_children {
        (
            quote! { const FIELDS: [&str; 3] = ["type", "attributes", "children"]; },
            quote! { formatter.write_str("an map with properties type, attributes and children") },
        )
    } else if has_attributes {
        (
            quote! { const FIELDS: [&str; 2] = ["type", "attributes"]; },
            quote! { formatter.write_str("an map with properties type and attributes") },
        )
    } else if has_children {
        (
            quote! { const FIELDS: [&str; 2] = ["type", "children"]; },
            quote! { formatter.write_str("an map with properties type and children") },
        )
    } else {
        (
            quote! { const FIELDS: [&str; 1] = ["type"]; },
            quote! { formatter.write_str("an map with type property") },
        )
    };

    quote! {
        use serde::de::{Error, MapAccess};

        #fields

        #[derive(Default)]
        struct #visitor_ident;

        impl<'de> serde::de::Visitor<'de> for #visitor_ident {
            type Value = #struct_ident;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #formatter
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut result = <#struct_ident>::default();
                while let Some(key) = access.next_key::<String>()? {
                    if key == "type" {
                        let value = access.next_value::<String>()?;
                        if value != #element_ident {
                            return Err(M::Error::custom(format!(
                                "expected type to equal {}, found {}",
                                #element_ident,
                                value,
                            )));
                        }
                    }
                    #set_attributes
                    #set_children
                    else {
                        return Err(M::Error::unknown_field(&key, &FIELDS));
                    }
                }
                Ok(result)
            }
        }

        impl<'de> serde::Deserialize<'de> for #struct_ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_map(<#visitor_ident>::default())
            }
        }
    }
}

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    let opts = Opts::from_derive_input(&ast).expect("Wrong options");

    let serializer = create_serializer(&ast, &opts);
    let deserializer = create_deserialize(&ast, &opts);

    quote! {
        #serializer
        #deserializer
    }
    .into()
}
