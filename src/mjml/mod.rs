#![allow(dead_code)]

use crate::mj_body::MjBody;
use crate::mj_head::MjHead;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
pub mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mjml";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintAttributes))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseAttributes))]
pub struct MjmlAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub struct MjmlChildren {
    head: Option<MjHead>,
    body: Option<MjBody>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct Mjml {
    pub attributes: MjmlAttributes,
    pub children: MjmlChildren,
}

impl Mjml {
    pub fn body(&self) -> Option<&MjBody> {
        self.children.body.as_ref()
    }

    pub fn head(&self) -> Option<&MjHead> {
        self.children.head.as_ref()
    }
}
